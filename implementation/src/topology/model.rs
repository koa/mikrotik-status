use std::collections::HashMap;
use std::net::IpAddr;
use std::ops::Deref;
use std::sync::Arc;

use ipnet::{Ipv4Net, Ipv6Net};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TopologyError {
    #[error("No device with id {0} defined")]
    MissingDeviceReference(usize),
    #[error("Device {device_idx} has no port {port_idx}")]
    MissingPortReference { device_idx: usize, port_idx: usize },
    #[error("Cannot connect port of device {last_device} to port of device {current_device}")]
    InvalidPath {
        last_device: usize,
        current_device: usize,
    },
}

#[derive(Debug)]
pub struct Topology {
    devices: Vec<Arc<Device>>,
    links: Vec<Arc<Link>>,
    link_index: HashMap<PortIdx, usize>,
    device_index: HashMap<u32, usize>,
}

impl Topology {
    pub fn builder() -> TopologyBuilder {
        TopologyBuilder::default()
    }
    pub fn get_device(self: &Arc<Self>, idx: usize) -> Option<DeviceRef> {
        self.devices.get(idx).map(|found| DeviceRef {
            topology: self.clone(),
            device: found.clone(),
            device_idx: idx,
        })
    }
    pub fn get_device_by_id(self: &Arc<Self>, key: u32) -> Option<DeviceRef> {
        self.get_device(*self.device_index.get(&key)?)
    }
    pub fn list_devices(self: &Arc<Self>) -> Vec<DeviceRef> {
        self.list_devices_filtered(|_| true)
    }
    pub fn list_devices_filtered<P: FnMut(&DeviceRef) -> bool>(
        self: &Arc<Self>,
        filter: P,
    ) -> Vec<DeviceRef> {
        self.devices
            .iter()
            .enumerate()
            .map(|(idx, found)| DeviceRef {
                topology: self.clone(),
                device: found.clone(),
                device_idx: idx,
            })
            .filter(filter)
            .collect()
    }
    pub fn list_devices_map<P: Fn(DeviceRef) -> Option<T>, T>(
        self: &Arc<Self>,
        filter: P,
    ) -> Vec<T> {
        self.devices
            .iter()
            .enumerate()
            .map(|(idx, found)| DeviceRef {
                topology: self.clone(),
                device: found.clone(),
                device_idx: idx,
            })
            .flat_map(filter)
            .collect()
    }
}

#[derive(Default)]
pub struct TopologyBuilder {
    devices: Vec<Device>,
    links: Vec<Link>,
}

impl TopologyBuilder {
    pub fn append_device(&mut self, id: u32, name: String, has_routeros: bool) -> DeviceBuilder {
        DeviceBuilder {
            topo_builder: self,
            id,
            name,
            ports: vec![],
            has_routeros,
        }
    }
    pub fn append_link(&mut self) -> LinkBuilder {
        LinkBuilder {
            topo_builder: self,
            path: vec![],
        }
    }
    pub fn build(self) -> Arc<Topology> {
        let mut links = Vec::with_capacity(self.links.len());
        let mut link_index = HashMap::new();
        for (link_idx, link) in self.links.into_iter().enumerate() {
            for segment in link.path.iter() {
                for port in segment.ports() {
                    link_index.entry(*port).or_insert(link_idx);
                }
            }
            links.push(Arc::new(link));
        }
        let mut devices = Vec::with_capacity(self.devices.len());
        let mut device_index = HashMap::new();
        for device in self.devices {
            device_index.insert(device.id.clone(), devices.len());
            devices.push(Arc::new(device));
        }
        Arc::new(Topology {
            devices,
            links,
            link_index,
            device_index,
        })
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Device {
    name: String,
    id: u32,
    ports: Vec<Arc<DevicePort>>,
    has_routeros: bool,
}

impl Device {}

pub struct DeviceBuilder<'a> {
    topo_builder: &'a mut TopologyBuilder,
    id: u32,
    name: String,
    ports: Vec<DevicePort>,
    has_routeros: bool,
}

impl DeviceBuilder<'_> {
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
    pub(crate) fn build(self) -> usize {
        let device = Device {
            id: self.id,
            name: self.name,
            ports: self.ports.into_iter().map(Arc::new).collect(),
            has_routeros: self.has_routeros,
        };
        self.topo_builder.devices.push(device);
        self.topo_builder.devices.len() - 1
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
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Link {
    path: Vec<LinkSegment>,
}

pub struct LinkBuilder<'a> {
    topo_builder: &'a mut TopologyBuilder,
    path: Vec<LinkSegment>,
}

impl LinkBuilder<'_> {
    pub fn append_segment(
        &mut self,
        left_device_idx: usize,
        left_port: usize,
        right_device_idx: usize,
        right_port: usize,
    ) -> Result<usize, TopologyError> {
        let left_device = self
            .topo_builder
            .devices
            .get(left_device_idx)
            .ok_or(TopologyError::MissingDeviceReference(left_device_idx))?;
        if left_device.ports.len() <= left_port {
            return Err(TopologyError::MissingPortReference {
                device_idx: left_device_idx,
                port_idx: left_device_idx,
            });
        }
        let right_device = self
            .topo_builder
            .devices
            .get(right_device_idx)
            .ok_or(TopologyError::MissingDeviceReference(right_device_idx))?;
        if right_device.ports.len() <= right_port {
            return Err(TopologyError::MissingPortReference {
                device_idx: right_device_idx,
                port_idx: right_device_idx,
            });
        }
        if let Some(last_segment) = self.path.last() {
            let last_device = last_segment.right_port.device_idx;
            if last_device != left_device_idx {
                return Err(TopologyError::InvalidPath {
                    last_device,
                    current_device: left_device_idx,
                });
            }
        }
        self.path.push(LinkSegment {
            left_port: PortIdx {
                device_idx: left_device_idx,
                port_idx: left_port,
            },
            right_port: PortIdx {
                device_idx: right_device_idx,
                port_idx: right_port,
            },
        });
        Ok(self.path.len() - 1)
    }
    pub fn build(self) -> usize {
        self.topo_builder.links.push(Link { path: self.path });
        self.topo_builder.links.len() - 1
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct PortIdx {
    device_idx: usize,
    port_idx: usize,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct LinkSegment {
    left_port: PortIdx,
    right_port: PortIdx,
}

impl LinkSegment {
    fn ports(&self) -> [&PortIdx; 2] {
        [&self.left_port, &self.right_port]
    }
}

#[derive(Clone, Debug)]
pub struct DeviceRef {
    topology: Arc<Topology>,
    device: Arc<Device>,
    device_idx: usize,
}

impl DeviceRef {
    pub(crate) fn has_routeros(&self) -> bool {
        self.device.has_routeros
    }
}

impl DeviceRef {
    pub(crate) fn get_loopback_address(&self) -> Option<IpAddr> {
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
}

impl DeviceRef {
    pub(crate) fn get_id(&self) -> u32 {
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
            .map(|(idx, port)| {
                Arc::new(DevicePortRef {
                    device: self.clone(),
                    port: port.clone(),
                    idx,
                })
            })
            .collect()
    }
}

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
                for (idx, segment) in link.path.iter().enumerate() {
                    if segment.right_port == port_idx {
                        hit_idx.push((PortSide::Left, idx));
                    }
                    if segment.left_port == port_idx {
                        hit_idx.push((PortSide::Right, idx));
                    }
                }
                hit_idx.shrink_to_fit();
                Arc::new(LinkPortRef {
                    topology: self.device.topology.clone(),
                    link,
                    hit_idx,
                })
            })
            .next()
    }
    pub fn get_device(self: &Arc<Self>) -> Arc<DeviceRef> {
        self.device.clone()
    }
}

enum PortSide {
    Left,
    Right,
}

pub struct LinkPortRef {
    topology: Arc<Topology>,
    link: Arc<Link>,
    hit_idx: Vec<(PortSide, usize)>,
}

#[cfg(test)]
mod tests {
    use crate::topology::model::{DevicePort, Topology};

    #[test]
    fn test_build_topology() {
        let mut topology_builder = Topology::builder();
        let mut rt01_ports = Vec::new();
        let rt01_idx = {
            let mut device_builder = topology_builder.append_device(1, "rt01".to_string(), true);
            device_builder.append_interface(
                1,
                "loopback".to_string(),
                Some("172.16.0.1/32".parse().unwrap()),
                None,
                true,
            );
            for port_idx in 1..=8 {
                device_builder.append_interface(
                    port_idx + 1,
                    format!("e{port_idx:02}"),
                    None,
                    None,
                    false,
                );
            }
            device_builder.build()
        };
        let mut rt02_ports = Vec::new();
        let rt02_idx = {
            let mut device_builder = topology_builder.append_device(2, "rt02".to_string(), true);
            device_builder.append_interface(
                10,
                "loopback".to_string(),
                Some("172.16.0.2/32".parse().unwrap()),
                None,
                true,
            );
            for port_idx in 1..=8 {
                device_builder.append_interface(
                    port_idx + 11,
                    format!("e{port_idx:02}"),
                    None,
                    None,
                    false,
                );
            }

            device_builder.build()
        };
        let mut link_builder = topology_builder.append_link();
        link_builder
            .append_segment(rt01_idx, rt01_ports[0], rt02_idx, rt02_ports[0])
            .unwrap();
        link_builder.build();

        let topology = topology_builder.build();

        println!("Topology: {topology:#?}");

        topology.get_device(2);
    }
}
