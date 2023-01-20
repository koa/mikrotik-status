use std::sync::Arc;

use async_graphql::Object;

use crate::api::location::Location;
use crate::error::BackendError;
use crate::topology::model;
use crate::topology::model::Topology;
use crate::topology::query::get_topology;

#[derive(Debug)]
pub struct Site {
    site: Arc<model::Site>,
    topology: Arc<Topology>,
}

impl Site {
    pub fn new(site: Arc<model::Site>, topology: Arc<Topology>) -> Self {
        Self { site, topology }
    }
}

pub async fn get_site(id: u32) -> Result<Option<Site>, BackendError> {
    let topology = get_topology().await?;
    Ok(topology
        .get_site_by_id(id)
        .map(|s| Site::new(s.clone(), topology.clone())))
}
pub async fn list_sites() -> Result<Vec<Site>, BackendError> {
    let topology = get_topology().await?;
    Ok(topology.list_sites_map(|s| Some(Site::new(s.clone(), topology.clone()))))
}

#[Object]
impl Site {
    async fn id(&self) -> u32 {
        self.site.id()
    }
    async fn name(&self) -> &str {
        self.site.name()
    }
    async fn address(&self) -> Vec<&str> {
        self.site
            .address()
            .split('\n')
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect()
    }
    async fn locations(&self) -> Vec<Location> {
        self.site
            .locations()
            .iter()
            .flat_map(|id| self.topology.get_location(*id))
            .map(|l| Location::new(l, self.topology.clone()))
            .collect()
    }
    async fn count_locations(&self) -> usize {
        self.site.locations().len()
    }
}
