use async_graphql::Object;

use crate::topology::model::device_type::DeviceTypRef;

#[derive(Debug)]
pub struct DeviceType(DeviceTypRef);

impl DeviceType {
    pub fn new(device_type: DeviceTypRef) -> Self {
        Self(device_type)
    }
}

#[Object]
impl DeviceType {
    async fn id(&self) -> u32 {
        self.0.id()
    }
    async fn name(&self) -> &str {
        self.0.name()
    }
    async fn has_routeros(&self) -> bool {
        self.0.has_routeros()
    }
}
