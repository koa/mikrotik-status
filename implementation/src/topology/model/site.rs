use std::sync::Arc;

use crate::topology::model::location::{Location, LocationRef};
use crate::topology::model::{Topology, TopologyBuilder};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Site {
    id: u32,
    name: String,
    address: String,
    locations: Vec<usize>,
}

impl Site {
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn address(&self) -> &str {
        &self.address
    }
    pub fn locations(&self) -> &Vec<usize> {
        &self.locations
    }
}

pub struct SiteBuilder<'a> {
    topo_builder: &'a mut TopologyBuilder,
    id: u32,
    name: String,
    address: String,
    locations: Vec<usize>,
}

impl<'a> SiteBuilder<'a> {
    pub(crate) fn append_location(&mut self, id: u32, name: String) {
        self.topo_builder.locations.push(Location::new(id, name));
        let idx = self.topo_builder.locations.len();
        self.locations.push(idx);
    }
    pub fn append_location_id(&mut self, idx: usize) -> &mut Self {
        self.locations.push(idx);
        self
    }
    pub fn build(self) -> usize {
        let mut locations = self.locations;
        locations.shrink_to_fit();
        self.topo_builder.sites.push(Site {
            id: self.id,
            name: self.name,
            address: self.address,
            locations,
        });
        self.topo_builder.sites.len() - 1
    }
    pub fn new(
        topo_builder: &'a mut TopologyBuilder,
        id: u32,
        name: String,
        address: String,
    ) -> Self {
        Self {
            topo_builder,
            id,
            name,
            address,
            locations: vec![],
        }
    }
}

#[derive(Debug)]
pub struct SiteRef {
    topology: Arc<Topology>,
    site: Arc<Site>,
}

impl SiteRef {
    pub fn new(topology: Arc<Topology>, site: Arc<Site>) -> Self {
        Self { topology, site }
    }
    pub(crate) fn get_id(&self) -> u32 {
        self.site.id
    }
    pub fn get_name(&self) -> &str {
        &self.site.name
    }
    pub fn get_address(&self) -> &str {
        &self.site.address
    }
    pub fn locations(&self) -> Vec<LocationRef> {
        self.site
            .locations
            .iter()
            .filter_map(|location| self.topology.get_location(*location))
            .collect()
    }
    pub fn location(&self, id: usize) -> Option<LocationRef> {
        let location = self.site.locations.get(id)?;
        self.topology.get_location(*location)
    }
}
