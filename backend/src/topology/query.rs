use std::backtrace::Backtrace;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;

use cached::proc_macro::cached;
use graphql_client::{GraphQLQuery, Response};
use ipnet::IpNet;
use log::debug;
use reqwest::header::AUTHORIZATION;
use thiserror::Error;

use crate::config::config;
use crate::error::{BackendError, GraphqlError};
use crate::topology::graphql_operations::fetch_topology::IpamIPAddressRoleChoices;
use crate::topology::graphql_operations::FetchTopology;
use crate::topology::model::device::DeviceBuilder;
use crate::topology::model::device_type::DeviceType;
use crate::topology::model::Topology;

enum PortType {
    Interface,
    Front,
    Rear,
}

#[derive(Debug, Error, Clone)]
pub enum NetboxError {
    #[error("Unknown Port type: {0}")]
    UnknownPortType(String),
    #[error("Device {0} not found")]
    DeviceNotFound(u32),
}

impl FromStr for PortType {
    type Err = NetboxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "interface" => Ok(PortType::Interface),
            "frontport" => Ok(PortType::Front),
            "rearport" => Ok(PortType::Rear),
            unknown => Err(NetboxError::UnknownPortType(unknown.to_string())),
        }
    }
}

#[cached(result, time = 30, time_refresh)]
pub async fn get_topology() -> Result<Arc<Topology>, BackendError> {
    let mut topo_builder = Topology::builder();
    let mut device_id_map = HashMap::new();
    let mut device_interface_map = HashMap::new();
    let mut device_front_map = HashMap::new();
    let mut device_rear_map = HashMap::new();
    let mut devices_of_location: HashMap<_, Vec<_>> = HashMap::new();

    let netbox_topology = query_netbox::<FetchTopology>(Default::default()).await?;
    let mut routeros_device_types = HashSet::new();
    let flatten = netbox_topology.device_type_list.into_iter().flatten();
    for type_entry in flatten {
        let has_routeros = type_entry
            .tags
            .iter()
            .flatten()
            .flatten()
            .any(|option_tag| option_tag.slug == "routeros");
        let id = type_entry.id.parse()?;
        let name = type_entry.model;
        topo_builder.append_device_type(DeviceType::new(name, id, has_routeros));
        if has_routeros {
            routeros_device_types.insert(id);
        }
    }

    for device_entry in netbox_topology.device_list.into_iter().flatten() {
        let device_type_id = device_entry.device_type.id.parse()?;
        let has_routeros = routeros_device_types.contains(&device_type_id);

        let mut device_builder = DeviceBuilder::new(
            device_entry.id.parse()?,
            device_entry.name.clone().unwrap_or_default(),
            has_routeros,
        );
        device_builder.set_device_type(device_type_id);

        let mut if_idx = Vec::with_capacity(device_entry.interfaces.len());
        for if_port in device_entry.interfaces {
            let id: u32 = if_port.id.parse()?;
            let name = if_port.name;
            let mut ipv6_address = None;
            let mut ipv4_address = None;
            let mut is_loopback = false;
            for ip_address in if_port.ip_addresses.iter().flat_map(|a| a.iter()) {
                if let Some(address_entry) = ip_address.as_ref() {
                    if let Ok(address) = address_entry.address.parse() {
                        match address {
                            IpNet::V4(a) => ipv4_address = Some(a),
                            IpNet::V6(a) => ipv6_address = Some(a),
                        }
                    }
                    if let Some(IpamIPAddressRoleChoices::LOOPBACK) = address_entry.role.as_ref() {
                        is_loopback = true;
                    }
                }
            }
            if_idx.push((
                id,
                device_builder.append_interface(id, name, ipv4_address, ipv6_address, is_loopback),
            ));
        }
        let mut rear_idx_list = Vec::with_capacity(device_entry.frontports.len());
        let mut front_idx_list = Vec::with_capacity(device_entry.frontports.len());
        for front_port in device_entry.frontports {
            let rear_port = front_port.rear_port;
            let rear_id = rear_port.id.parse()?;
            let rear_idx = device_builder.append_rear_port(rear_id, rear_port.name);
            rear_idx_list.push((rear_id, rear_idx));
            let front_id = front_port.id.parse()?;
            let front_idx = device_builder.append_front_port(front_id, front_port.name, rear_idx);
            front_idx_list.push((front_id, front_idx));
        }

        let dev_idx = topo_builder.devices().len();
        if let Some(id) = device_entry
            .location
            .and_then(|location_of_device| location_of_device.id.parse::<u32>().ok())
        {
            device_builder.set_location(id);
            devices_of_location.entry(id).or_default().push(dev_idx);
        }
        if let Ok(site_id) = device_entry.site.id.parse::<u32>() {
            device_builder.set_site(site_id);
        }
        topo_builder.append_device(device_builder);
        for (port_id, port_idx) in if_idx {
            device_interface_map.insert(port_id, (dev_idx, port_idx));
        }
        for (port_id, port_idx) in rear_idx_list {
            device_rear_map.insert(port_id, (dev_idx, port_idx));
        }
        for (port_id, port_idx) in front_idx_list {
            device_front_map.insert(port_id, (dev_idx, port_idx));
        }
        device_id_map.insert(device_entry.id.clone(), dev_idx);
    }
    for site in netbox_topology.site_list.into_iter().flatten() {
        let id = site.id.parse()?;
        let name = site.name;
        let address = site.physical_address;
        let site_idx = topo_builder.append_site(id, name, address);
        for location in site.locations {
            let id = location.id.parse()?;
            let name = location.name;
            let mut devices = devices_of_location.remove(&id).unwrap_or_default();
            devices.shrink_to_fit();
            let location_idx = topo_builder.append_location(id, name, devices);
            topo_builder.set_site_of_location(location_idx, site_idx);
        }
    }

    topo_builder.build()
}

pub async fn query_netbox<Q>(request: Q::Variables) -> Result<Q::ResponseData, BackendError>
where
    Q: GraphQLQuery,
    Q::Variables: Debug,
    Q::ResponseData: Debug,
{
    let request_body = Q::build_query(request);
    let name = request_body.operation_name;
    debug!("Graphql Request {name}: {request_body:?}");
    let client = reqwest::Client::new();
    let config = config();
    let response: Response<Q::ResponseData> = client
        .post(config.netbox_endpoint())
        .json(&request_body)
        .header(AUTHORIZATION, format!("Token {}", config.netbox_token()))
        .send()
        .await?
        .json()
        .await?;
    if let Some(data) = response.data {
        debug!("Graphql Response {name}: {data:?}");
        Ok(data)
    } else {
        let error = GraphqlError::new(response.errors);
        debug!("Graphql Error {name}: {error:?}");
        Err(BackendError::Graphql {
            error: Arc::new(error),
            backtrace: Arc::new(Backtrace::force_capture()),
        })
    }
}
