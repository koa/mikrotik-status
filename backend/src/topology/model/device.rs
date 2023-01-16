use std::net::IpAddr;
use std::ops::Deref;
use std::sync::Arc;

use ipnet::{IpNet, Ipv4Net, Ipv6Net};

use crate::error::BackendError;
use crate::error::Result;
use crate::topology::model::device_type::DeviceTypRef;
use crate::topology::model::link::LinkPortRef;
use crate::topology::model::location::LocationRef;
use crate::topology::model::Topology;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Device {
    name: String,
    id: u32,
    ports: Vec<Arc<DevicePort>>,
    has_routeros: bool,
    location: Option<usize>,
    site: Option<usize>,
    device_type: usize,
    device_category: DeviceCategory,
}

impl Device {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn ports(&self) -> &Vec<Arc<DevicePort>> {
        &self.ports
    }
    pub fn has_routeros(&self) -> bool {
        self.has_routeros
    }
}

pub struct DeviceBuilder {
    id: u32,
    name: String,
    ports: Vec<DevicePort>,
    has_routeros: bool,
    site_id: Option<u32>,
    location_id: Option<u32>,
    device_type: Option<u32>,
    device_category: Option<DeviceCategory>,
}

impl DeviceBuilder {
    pub(crate) fn set_device_type(&mut self, device_type_id: u32) {
        self.device_type = Some(device_type_id);
    }

    pub fn append_interface(
        &mut self,
        id: u32,
        name: String,
        v4_address: Option<Ipv4Net>,
        v6_address: Option<Ipv6Net>,
        loopback: bool,
    ) -> usize {
        self.ports.push(DevicePort::Interface {
            id,
            name,
            v4_address,
            v6_address,
            loopback,
        });
        self.ports.len() - 1
    }
    pub fn append_front_port(&mut self, id: u32, name: String, rear_port_idx: usize) -> usize {
        self.ports.push(DevicePort::FrontPort {
            id,
            name,
            rear_port_idx,
        });
        self.ports.len() - 1
    }

    pub fn append_rear_port(&mut self, id: u32, name: String) -> usize {
        self.ports.push(DevicePort::RearPort { id, name });
        self.ports.len() - 1
    }

    pub fn append_port(&mut self, port: DevicePort) -> usize {
        self.ports.push(port);
        self.ports.len() - 1
    }

    pub fn set_location(&mut self, id: u32) {
        self.location_id = Some(id);
    }
    pub fn set_site(&mut self, id: u32) {
        self.location_id = Some(id);
    }
    pub fn set_category(&mut self, category: DeviceCategory) {
        self.device_category = Some(category);
    }

    pub(crate) fn build<LM, SM, TM>(
        self,
        location_mapper: &LM,
        site_mapper: &SM,
        type_mapper: &TM,
    ) -> Result<Device>
    where
        LM: Fn(u32) -> Option<usize>,
        SM: Fn(u32) -> Option<usize>,
        TM: Fn(u32) -> Option<usize>,
    {
        Ok(Device {
            id: self.id,
            name: self.name,
            ports: self.ports.into_iter().map(Arc::new).collect(),
            has_routeros: self.has_routeros,
            location: self.location_id.and_then(location_mapper),
            site: self.site_id.and_then(site_mapper),
            device_type: self
                .device_type
                .and_then(type_mapper)
                .ok_or(BackendError::MissingDeviceType())?,
            device_category: self.device_category.unwrap_or_default(),
        })
    }
    pub fn new(id: u32, name: String, has_routeros: bool) -> Self {
        Self {
            id,
            name,
            ports: vec![],
            has_routeros,
            site_id: None,
            location_id: None,
            device_type: None,
            device_category: None,
        }
    }
    pub fn ports(&self) -> &Vec<DevicePort> {
        &self.ports
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum DevicePort {
    Interface {
        id: u32,
        name: String,
        v4_address: Option<Ipv4Net>,
        v6_address: Option<Ipv6Net>,
        loopback: bool,
    },
    FrontPort {
        id: u32,
        name: String,
        rear_port_idx: usize,
    },
    RearPort {
        id: u32,
        name: String,
    },
}

impl DevicePort {
    pub fn get_name(&self) -> &str {
        match self {
            DevicePort::Interface { name, .. } => name,
            DevicePort::FrontPort { name, .. } => name,
            DevicePort::RearPort { name, .. } => name,
        }
    }
    pub fn list_nets(&self) -> Vec<IpNet> {
        match self {
            DevicePort::Interface {
                v4_address,
                v6_address,
                ..
            } => v4_address
                .map(IpNet::V4)
                .into_iter()
                .chain(v6_address.map(IpNet::V6).into_iter())
                .collect(),
            DevicePort::FrontPort { .. } => {
                vec![]
            }
            DevicePort::RearPort { .. } => {
                vec![]
            }
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct PortIdx {
    device_idx: usize,
    port_idx: usize,
}

impl PortIdx {
    pub fn new(device_idx: usize, port_idx: usize) -> Self {
        Self {
            device_idx,
            port_idx,
        }
    }

    pub fn device_idx(&self) -> usize {
        self.device_idx
    }
    pub fn port_idx(&self) -> usize {
        self.port_idx
    }
}

#[derive(Clone, Debug)]
pub struct DeviceRef {
    topology: Arc<Topology>,
    device: Arc<Device>,
    device_idx: usize,
}

impl DeviceRef {
    pub fn has_routeros(&self) -> bool {
        self.device.has_routeros
    }

    pub fn get_loopback_address(&self) -> Option<IpAddr> {
        self.device
            .ports
            .iter()
            .flat_map(|p| match p.deref() {
                DevicePort::Interface {
                    id: _,
                    name: _,
                    v4_address,
                    v6_address,
                    loopback: true,
                } => v6_address
                    .iter()
                    .map(|a| IpAddr::V6(a.addr()))
                    .chain(v4_address.iter().map(|a| IpAddr::V4(a.addr())))
                    .next(),
                _ => None,
            })
            .next()
    }

    pub fn get_id(&self) -> u32 {
        self.device.id
    }
    pub fn get_name(&self) -> &str {
        &self.device.name
    }
    pub fn get_ports(self: &Arc<Self>) -> Vec<Arc<DevicePortRef>> {
        self.device
            .ports
            .iter()
            .enumerate()
            .map(|(idx, port)| DevicePortRef {
                device: self.clone(),
                port: port.clone(),
                idx,
            })
            .map(Arc::new)
            .collect()
    }
    pub fn location(&self) -> Option<LocationRef> {
        self.device
            .location
            .and_then(|location_idx| self.topology.get_location(location_idx))
    }
    pub fn device_type(&self) -> Option<DeviceTypRef> {
        self.topology.get_device_type(self.device.device_type)
    }
    pub fn new(topology: Arc<Topology>, device: Arc<Device>, device_idx: usize) -> Self {
        Self {
            topology,
            device,
            device_idx,
        }
    }
}

#[derive(Debug)]
pub struct DevicePortRef {
    device: Arc<DeviceRef>,
    port: Arc<DevicePort>,
    idx: usize,
}

impl DevicePortRef {
    pub fn get_name<'a>(self: &'a Arc<Self>) -> &'a str {
        self.port.get_name()
    }
    pub fn get_links(self: &Arc<Self>) -> Option<Arc<LinkPortRef>> {
        let device_idx = self.device.device_idx;
        let port_idx = self.idx;
        let port_idx = PortIdx {
            device_idx,
            port_idx,
        };
        self.device
            .topology
            .link_index
            .get(&port_idx)
            .iter()
            .flat_map(|link_idx| self.device.topology.links.get(**link_idx).cloned())
            .map(|link| {
                let mut hit_idx = Vec::with_capacity(2);
                for (idx, segment) in link.path().iter().enumerate() {
                    if segment.right_port() == port_idx {
                        hit_idx.push((PortSide::Left, idx));
                    }
                    if segment.left_port() == port_idx {
                        hit_idx.push((PortSide::Right, idx));
                    }
                }
                hit_idx.shrink_to_fit();
                Arc::new(LinkPortRef::new(
                    self.device.topology.clone(),
                    link,
                    hit_idx,
                ))
            })
            .next()
    }
    pub fn get_device(self: &Arc<Self>) -> Arc<DeviceRef> {
        self.device.clone()
    }
    pub fn get_ips(&self) -> Vec<IpNet> {
        self.port.list_nets()
    }
}

pub enum PortSide {
    Left,
    Right,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug, Copy)]
pub enum DeviceCategory {
    Switch,
    Router,
    UserDevice,
    PatchPanel,
    Server,
    WallConnector,
    Unknown,
}

impl Default for DeviceCategory {
    fn default() -> Self {
        DeviceCategory::Unknown
    }
}

impl DeviceCategory {
    pub fn can_ping(&self) -> bool {
        match self {
            DeviceCategory::Switch => true,
            DeviceCategory::Router => true,
            DeviceCategory::UserDevice => true,
            DeviceCategory::PatchPanel => false,
            DeviceCategory::Server => true,
            DeviceCategory::WallConnector => false,
            DeviceCategory::Unknown => false,
        }
    }
}
