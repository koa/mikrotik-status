use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use cached::proc_macro::cached;
use ipnet::IpNet;

use crate::error::BackendError;
use crate::netbox::query;
use crate::topology::graphql_operations::list_all_devices::IpamIPAddressRoleChoices;
use crate::topology::graphql_operations::ListAllDevices;
use crate::topology::model::{Device, DevicePort, Topology};
use log::info;

#[cached(result, time = 30, time_refresh)]
pub async fn get_topology() -> Result<Arc<Topology>, BackendError> {
    info!("Fetch Topology");
    let device_list = query::<ListAllDevices>(Default::default()).await?;
    let routeros_device_types: HashSet<_> = device_list
        .device_type_list
        .iter()
        .flatten()
        .filter(|type_entry| {
            type_entry
                .tags
                .iter()
                .flatten()
                .flatten()
                .any(|option_tag| option_tag.slug == "routeros")
        })
        .map(|t| t.id.as_str())
        .collect();
    let mut topo_builder = Topology::builder();
    let mut device_id_map = HashMap::new();
    for device_entry in device_list.device_list.iter().flatten() {
        let has_routeros = routeros_device_types.contains(device_entry.device_type.id.as_str());
        let mut device_builder = Device::builder(
            device_entry.id.parse()?,
            device_entry.name.clone().unwrap_or_default(),
            has_routeros,
        );
        for if_port in device_entry.interfaces.iter() {
            let name = &if_port.name;
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
            device_builder.append_port(DevicePort::new_interface(
                name,
                ipv4_address,
                ipv6_address,
                is_loopback,
            ));
        }

        topo_builder.append_device(device_builder.build());
        device_id_map.insert(device_entry.id.clone(), 0);
    }
    Ok(topo_builder.build())
}
