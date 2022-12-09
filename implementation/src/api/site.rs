use async_graphql::Object;

use crate::error::BackendError;
use crate::topology::model::SiteRef;
use crate::topology::query::get_topology;

#[derive(Debug)]
pub struct Site(SiteRef);

impl Site {
    fn new(s: SiteRef) -> Site {
        Site(s)
    }
}

pub async fn get_site(id: u32) -> Result<Option<Site>, BackendError> {
    let topology = get_topology().await?;
    Ok(topology.get_site_by_id(id).map(|s| Site::new(s)))
}
pub async fn list_sites() -> Result<Vec<Site>, BackendError> {
    let topology = get_topology().await?;
    Ok(topology.list_sites_map(|s| Some(Site::new(s))))
}

#[Object]
impl Site {
    async fn id(&self) -> u32 {
        self.0.get_id()
    }
    async fn name(&self) -> &str {
        self.0.get_name()
    }
    async fn address(&self) -> Vec<&str> {
        self.0
            .get_address()
            .split('\n')
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect()
    }
}