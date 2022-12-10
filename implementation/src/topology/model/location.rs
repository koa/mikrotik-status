use std::sync::Arc;

use crate::topology::model::Topology;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Location {
    id: u32,
    name: String,
}

#[derive(Clone, Debug)]
pub struct LocationRef {
    topology: Arc<Topology>,
    location: Arc<Location>,
}

impl Location {
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
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
}
