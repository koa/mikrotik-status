use std::sync::Arc;

use async_graphql::Object;

use crate::api::device::Device;
use crate::api::site::Site;
use crate::error::BackendError;
use crate::topology::model;
use crate::topology::model::Topology;
use crate::topology::query::get_topology;

#[derive(Debug)]
pub struct Location {
    location: Arc<model::Location>,
    topology: Arc<Topology>,
}

impl Location {
    pub fn new(location: Arc<model::Location>, topology: Arc<Topology>) -> Location {
        Location { location, topology }
    }
}
pub async fn list_locations() -> Result<Vec<Location>, BackendError> {
    let topology = get_topology().await?;
    Ok(topology.list_locations_map(|s| Some(Location::new(s.clone(), topology.clone()))))
}

pub async fn get_location(id: u32) -> Result<Option<Location>, BackendError> {
    let topology = get_topology().await?;
    Ok(topology
        .get_location_by_id(id)
        .map(|l| Location::new(l, topology.clone())))
}

#[Object]
impl Location {
    /// id of location
    async fn id(&self) -> u32 {
        self.location.id()
    }
    /// name of location
    async fn name(&self) -> &str {
        self.location.name()
    }
    /// site of location (if there is any)
    async fn site(&self) -> Option<Site> {
        self.location
            .site()
            .and_then(|sid| self.topology.get_site(sid))
            .map(|s| Site::new(s.clone(), self.topology.clone()))
    }
    /// devices on that location
    async fn devices(&self) -> Vec<Device> {
        let topology = &self.topology;
        self.location
            .devices()
            .iter()
            .copied()
            .flat_map(|did| topology.get_device(did))
            .map(|d| Device::new(d, topology.clone()))
            .collect()
    }
}
