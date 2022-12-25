use async_graphql::Object;

use crate::api::device::{get_device, list_devices, Device};
use crate::api::location::Location;
use crate::api::location::{get_location, list_locations};
use crate::api::settings::SettingsData;
use crate::api::site::Site;
use crate::api::site::{get_site, list_sites};
use crate::error;
use crate::error::BackendError;

pub struct Query;

#[Object]
impl Query {
    /// gives the coordinates for authentication
    async fn settings(&self) -> error::Result<SettingsData> {
        SettingsData::create_from_config()
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
        list_sites().await
    }
    /// get single site by id
    async fn site(&self, id: u32) -> Result<Option<Site>, BackendError> {
        get_site(id).await
    }
    /// list all known locations
    async fn locations(&self) -> Result<Vec<Location>, BackendError> {
        list_locations().await
    }
    /// get single location
    async fn location(&self, id: u32) -> Result<Option<Location>, BackendError> {
        get_location(id).await
    }
}
