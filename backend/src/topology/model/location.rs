use std::sync::Arc;

use crate::topology::model::Topology;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Location {
    id: u32,
    name: String,
    site: Option<usize>,
}

pub struct LocationBuilder {
    id: u32,
    name: String,
    site: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct LocationRef {
    topology: Arc<Topology>,
    location: Arc<Location>,
}

impl LocationBuilder {
    pub fn site(&mut self, site_idx: usize) -> &mut Self {
        self.site = Some(site_idx);
        self
    }
    pub fn build(self) -> Location {
        Location {
            id: self.id,
            name: self.name,
            site: self.site,
        }
    }
}

impl Location {
    pub fn builder(id: u32, name: String) -> LocationBuilder {
        LocationBuilder {
            id,
            name,
            site: None,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn site(&self) -> Option<usize> {
        self.site
    }
}

impl LocationRef {
    pub fn new(topology: Arc<Topology>, location: Arc<Location>) -> Self {
        Self { topology, location }
    }
    pub fn id(&self) -> u32 {
        self.location.id
    }
    pub fn name(&self) -> &str {
        &self.location.name
    }
    pub fn site_id(&self) -> Option<usize> {
        self.location.site
    }
}
