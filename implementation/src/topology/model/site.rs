use std::sync::Arc;

use crate::topology::model::location::Location;
use crate::topology::model::{Topology, TopologyBuilder};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Site {
    id: u32,
    name: String,
    address: String,
    locations: Vec<Location>,
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
    pub fn locations(&self) -> &Vec<Location> {
        &self.locations
    }
}

pub struct SiteBuilder<'a> {
    topo_builder: &'a mut TopologyBuilder,
    id: u32,
    name: String,
    address: String,
    locations: Vec<Location>,
}

impl<'a> SiteBuilder<'a> {
    pub fn append_location(&mut self, id: u32, name: String) -> usize {
        self.locations.push(Location::new(id, name));
        self.locations.len() - 1
    }
    pub fn build(self) -> usize {
        self.topo_builder.sites.push(Site {
            id: self.id,
            name: self.name,
            address: self.address,
            locations: self.locations,
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
    site_idx: usize,
}

impl SiteRef {
    pub(crate) fn get_id(&self) -> u32 {
        self.site.id
    }
    pub fn get_name(&self) -> &str {
        &self.site.name
    }
    pub fn get_address(&self) -> &str {
        &self.site.address
    }
    pub fn new(topology: Arc<Topology>, site: Arc<Site>, site_idx: usize) -> Self {
        Self {
            topology,
            site,
            site_idx,
        }
    }
}
