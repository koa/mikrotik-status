use async_graphql::Object;

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

#[Object]
impl Location {
    async fn id(&self) -> u32 {
        self.0.id()
    }
    async fn name(&self) -> &str {
        self.0.name()
    }
}
