use async_graphql::Object;

use crate::api::device::{get_device, list_devices, Device};
use crate::api::settings::SettingsData;
use crate::error::BackendError;

pub struct Query;

#[Object]
impl Query {
    /// gives the coordinates for authentication
    async fn settings(&self) -> SettingsData {
        SettingsData::default()
    }
    /// list all known devices available for query
    async fn devices(&self) -> Result<Vec<Device>, BackendError> {
        list_devices().await
    }
    async fn device(&self, id: i64) -> Result<Option<Device>, BackendError> {
        get_device(id).await
    }
}
