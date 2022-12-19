use async_graphql::Object;

use crate::api::site::Site;
use crate::error::BackendError;
use crate::topology::model::location::LocationRef;
use crate::topology::query::get_topology;

#[derive(Debug)]
pub struct Location(LocationRef);

impl Location {
    pub fn new(s: LocationRef) -> Location {
        Location(s)
    }
}
pub async fn list_locations() -> Result<Vec<Location>, BackendError> {
    let topology = get_topology().await?;
    Ok(topology.list_locations_map(|s| Some(Location::new(s))))
}

pub async fn get_location(id: u32) -> Result<Option<Location>, BackendError> {
    let topology = get_topology().await?;
    Ok(topology.get_location_by_id(id).map(Location::new))
}

#[Object]
impl Location {
    /// id of location
    async fn id(&self) -> u32 {
        self.0.id()
    }
    /// name of location
    async fn name(&self) -> &str {
        self.0.name()
    }
    /// site of location (if there is any)
    async fn site(&self) -> Result<Option<Site>, BackendError> {
        let topology = get_topology().await?;
        let Some(site_id) = self.0.site_id()  else {
            return Ok(None);
        };
        let Some(site) = topology.get_site(site_id)  else {
          return  Ok(None);
        };
        Ok(Some(site.into()))
    }
}
