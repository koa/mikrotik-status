use std::sync::Arc;

use async_graphql::Object;

use crate::topology::model;
use crate::topology::model::Topology;

#[derive(Debug)]
pub struct DeviceType {
    device_type: Arc<model::DeviceType>,
    topology: Arc<Topology>,
}

impl DeviceType {
    pub fn new(device_type: Arc<model::DeviceType>, topology: Arc<Topology>) -> Self {
        Self {
            device_type,
            topology,
        }
    }
}

#[Object]
impl DeviceType {
    async fn id(&self) -> u32 {
        self.device_type.id()
    }
    async fn name(&self) -> &str {
        self.device_type.name()
    }
    async fn has_routeros(&self) -> bool {
        self.device_type.has_routeros()
    }
}
