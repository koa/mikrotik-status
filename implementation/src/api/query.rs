use async_graphql::Object;
use log::info;

use crate::api::device::{get_device, list_devices, Device};
use crate::api::settings::SettingsData;
use crate::api::site::list_sites;
use crate::api::site::Site;
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
    /// take single device by its id
    async fn device(&self, id: u32) -> Result<Option<Device>, BackendError> {
        get_device(id).await
    }
    /// list all known sites available for query
    async fn sites(&self) -> Result<Vec<Site>, BackendError> {
        let result = list_sites().await;
        info!("sites: {result:#?}");
        result
    }
}
