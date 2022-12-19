use std::collections::HashSet;
use std::sync::Arc;

use crate::topology::model::location::LocationRef;
use crate::topology::model::Topology;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Site {
    id: u32,
    name: String,
    address: String,
    locations: Vec<usize>,
}

impl Site {
    pub(crate) fn new(id: u32, name: String, address: String, locations: HashSet<usize>) -> Site {
        Site {
            id,
            name,
            address,
            locations: locations.into_iter().collect(),
        }
    }
    pub fn builder(id: u32, name: String, address: String) -> SiteBuilder {
        SiteBuilder {
            id,
            name,
            address,
            locations: vec![],
        }
    }
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

pub struct SiteBuilder {
    id: u32,
    name: String,
    address: String,
    locations: Vec<(u32, String)>,
}

impl SiteBuilder {
    pub fn new(id: u32, name: String, address: String) -> Self {
        Self {
            id,
            name,
            address,
            locations: vec![],
        }
    }
    pub(crate) fn append_location(&mut self, id: u32, name: String) {
        self.locations.push((id, name));
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn destruct(self) -> (u32, String, String) {
        (self.id, self.name, self.address)
    }
    pub fn locations(&self) -> &Vec<(u32, String)> {
        &self.locations
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
