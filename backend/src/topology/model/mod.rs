use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use log::warn;
use thiserror::Error;

pub use device::{Device, DevicePort, PortIdx};
use link::Link;
pub use site::Site;
use site::SiteBuilder;

use crate::error::Result;
use crate::topology::model::device::DeviceBuilder;
pub use crate::topology::model::device_type::DeviceType;
pub use crate::topology::model::location::Location;
use crate::topology::model::location::LocationBuilder;

pub mod device;
pub mod device_type;
pub mod link;
pub mod location;
pub mod site;

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
    device_types: Vec<Arc<DeviceType>>,
    devices: Vec<Arc<Device>>,
    links: Vec<Arc<Link>>,
    sites: Vec<Arc<Site>>,
    locations: Vec<Arc<Location>>,
    link_index: HashMap<PortIdx, usize>,
    device_index: HashMap<u32, usize>,
    site_index: HashMap<u32, usize>,
    location_index: HashMap<u32, usize>,
}

impl Topology {
    pub fn builder() -> TopologyBuilder {
        TopologyBuilder::default()
    }
    pub fn get_device(self: &Arc<Self>, idx: usize) -> Option<Arc<Device>> {
        self.devices.get(idx).cloned()
    }
    pub fn get_device_type(self: &Arc<Self>, idx: usize) -> Option<Arc<DeviceType>> {
        self.device_types.get(idx).cloned()
    }
    pub fn get_device_by_id(self: &Arc<Self>, key: u32) -> Option<Arc<Device>> {
        self.get_device(*self.device_index.get(&key)?)
    }
    pub fn list_devices(self: &Arc<Self>) -> Vec<Arc<Device>> {
        self.devices.clone()
    }

    pub fn list_devices_map<P: Fn(&Arc<Device>) -> Option<T>, T>(
        self: &Arc<Self>,
        filter: P,
    ) -> Vec<T> {
        self.devices.iter().flat_map(filter).collect()
    }
    pub fn get_site<'a>(self: &'a Arc<Self>, idx: usize) -> Option<&'a Arc<Site>> {
        self.sites.get(idx)
    }
    pub fn get_site_by_id<'a>(self: &'a Arc<Self>, key: u32) -> Option<&'a Arc<Site>> {
        self.get_site(*self.site_index.get(&key)?)
    }
    pub fn list_sites(self: &Arc<Self>) -> Vec<Arc<Site>> {
        self.list_sites_map(|s| Some(s.clone()))
    }
    pub fn list_sites_map<P: Fn(&Arc<Site>) -> Option<T>, T>(
        self: &Arc<Self>,
        filter: P,
    ) -> Vec<T> {
        self.sites.iter().flat_map(filter).collect()
    }
    pub fn get_location(self: &Arc<Self>, idx: usize) -> Option<Arc<Location>> {
        self.locations.get(idx).cloned()
    }
    pub fn list_locations_map<P: Fn(&Arc<Location>) -> Option<T>, T>(
        self: &Arc<Self>,
        filter: P,
    ) -> Vec<T> {
        self.locations.iter().flat_map(filter).collect()
    }
    pub fn get_location_by_id(self: &Arc<Self>, key: u32) -> Option<Arc<Location>> {
        self.get_location(self.location_index.get(&key)?.clone())
    }
}

#[derive(Default)]
pub struct TopologyBuilder {
    device_types: Vec<DeviceType>,
    devices: Vec<DeviceBuilder>,
    links: Vec<Link>,
    sites: Vec<SiteBuilder>,
    locations: Vec<LocationBuilder>,
}

impl TopologyBuilder {
    pub fn append_device_type(&mut self, device_type: DeviceType) -> usize {
        self.device_types.push(device_type);
        self.device_types.len() - 1
    }
    pub fn append_device(&mut self, device: DeviceBuilder) -> usize {
        self.devices.push(device);
        self.devices.len() - 1
    }
    pub fn append_link(&mut self, link: Link) -> usize {
        self.links.push(link);
        self.links.len() - 1
    }
    pub fn append_site(&mut self, id: u32, name: String, address: String) -> usize {
        self.sites.push(Site::builder(id, name, address));
        self.sites.len() - 1
    }
    pub fn append_location(&mut self, id: u32, name: String, devices: Vec<usize>) -> usize {
        self.locations.push(Location::builder(id, name, devices));
        self.locations.len() - 1
    }

    pub fn set_site_of_location(&mut self, location_idx: usize, site_idx: usize) {
        Self::modify_site_of_location(&mut self.locations, location_idx, site_idx)
    }

    fn modify_site_of_location(
        locations: &mut [LocationBuilder],
        location_idx: usize,
        site_idx: usize,
    ) {
        if let Some(location) = locations.get_mut(location_idx) {
            location.site(site_idx);
        } else {
            warn!("Location {location_idx} not found");
        }
    }

    pub fn build(mut self) -> Result<Arc<Topology>> {
        let mut device_types = Vec::with_capacity(self.device_types.len());
        let mut device_type_index = HashMap::new();
        for (idx, device_type) in self.device_types.into_iter().enumerate() {
            device_type_index.insert(device_type.id(), idx);
            device_types.push(Arc::new(device_type));
        }

        let mut locations = Vec::with_capacity(self.locations.len());
        let mut location_index = HashMap::new();
        let mut locations_of_site: HashMap<_, HashSet<usize>> = HashMap::new();
        for (site_idx, site_builder) in self.sites.iter().enumerate() {
            for (location_id, location_name) in site_builder.locations().iter().cloned() {
                let location_idx = self.locations.len();
                self.locations
                    .push(Location::builder(location_id, location_name, vec![]));
                Self::modify_site_of_location(&mut self.locations, location_idx, site_idx);
            }
        }
        for location in self.locations {
            let location_idx = locations.len();
            let location = location.build();
            if let Some(site_idx) = location.site() {
                locations_of_site
                    .entry(site_idx)
                    .or_default()
                    .insert(location_idx);
            }
            location_index.insert(location.id(), location_idx);
            locations.push(Arc::new(location));
        }

        let mut sites = Vec::with_capacity(self.sites.len());
        let mut site_index = HashMap::new();
        for (site_idx, site) in self.sites.into_iter().enumerate() {
            let locations = locations_of_site.remove(&site_idx).unwrap_or_default();
            let (id, name, address) = site.destruct();
            sites.push(Arc::new(Site::new(id, name, address, locations)));
            site_index.insert(id, site_idx);
        }
        let mut devices = Vec::with_capacity(self.devices.len());
        let mut device_index = HashMap::new();
        let location_mapper = |id| location_index.get(&id).copied();
        let site_mapper = |id| site_index.get(&id).copied();
        let type_mapper = |id| device_type_index.get(&id).copied();
        for device_builder in self.devices {
            let device = device_builder.build(&location_mapper, &site_mapper, &type_mapper)?;
            device_index.insert(device.id(), devices.len());
            devices.push(Arc::new(device));
        }
        let mut links = Vec::with_capacity(self.links.len());
        let mut link_index = HashMap::new();
        for (link_idx, link) in self.links.into_iter().enumerate() {
            for segment in link.path().iter() {
                for port in segment.ports() {
                    link_index.entry(*port).or_insert(link_idx);
                }
            }
            links.push(Arc::new(link));
        }

        Ok(Arc::new(Topology {
            device_types,
            devices,
            links,
            sites,
            locations,
            link_index,
            device_index,
            site_index,
            location_index,
        }))
    }
    pub fn devices(&self) -> &Vec<DeviceBuilder> {
        &self.devices
    }
}

#[cfg(test)]
mod tests {
    use crate::topology::model::device::DeviceBuilder;
    use crate::topology::model::link::LinkBuilder;
    use crate::topology::model::Topology;

    #[test]
    fn test_build_topology() {
        let mut topology_builder = Topology::builder();
        let mut rt01_ports = Vec::new();
        let rt01_idx = {
            let mut device_builder = DeviceBuilder::new(1, "rt01".to_string(), true);
            device_builder.append_interface(
                1,
                "loopback".to_string(),
                Some("172.16.0.1/32".parse().unwrap()),
                None,
                true,
            );
            for port_idx in 1..=8 {
                rt01_ports.push(device_builder.append_interface(
                    port_idx + 1,
                    format!("e{port_idx:02}"),
                    None,
                    None,
                    false,
                ));
            }
            topology_builder.append_device(device_builder)
        };
        let mut rt02_ports = Vec::new();
        let rt02_idx = {
            let mut device_builder = DeviceBuilder::new(2, "rt02".to_string(), true);
            device_builder.append_interface(
                10,
                "loopback".to_string(),
                Some("172.16.0.2/32".parse().unwrap()),
                None,
                true,
            );
            for port_idx in 1..=8 {
                rt02_ports.push(device_builder.append_interface(
                    port_idx + 11,
                    format!("e{port_idx:02}"),
                    None,
                    None,
                    false,
                ));
            }
            topology_builder.append_device(device_builder)
        };

        let mut link_builder = LinkBuilder::new();
        link_builder
            .append_segment(
                topology_builder.devices(),
                rt01_idx,
                rt01_ports[0],
                rt02_idx,
                rt02_ports[0],
            )
            .unwrap();
        topology_builder.append_link(link_builder.build());

        let topology = topology_builder.build();

        println!("Topology: {topology:#?}");

        topology.get_device(2);
    }
}
