use std::sync::Arc;

use crate::topology::model::Topology;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct DeviceType {
    name: String,
    id: u32,
    has_routeros: bool,
}

impl DeviceType {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn new(name: String, id: u32, has_routeros: bool) -> Self {
        Self {
            name,
            id,
            has_routeros,
        }
    }
}

#[derive(Debug)]
pub struct DeviceTypRef {
    topology: Arc<Topology>,
    device_type: Arc<DeviceType>,
    device_idx: usize,
}

impl DeviceTypRef {
    pub(crate) fn new(
        topology: Arc<Topology>,
        device_type: Arc<DeviceType>,
        device_idx: usize,
    ) -> Self {
        Self {
            topology,
            device_type,
            device_idx,
        }
    }
    pub fn name(&self) -> &str {
        &self.device_type.name
    }
    pub fn id(&self) -> u32 {
        self.device_type.id
    }
    pub fn has_routeros(&self) -> bool {
        self.device_type.has_routeros
    }
}
