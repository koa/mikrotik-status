use std::num::TryFromIntError;
use std::ops::Deref;
use std::rc::Rc;

use cached::instant::Instant;
use cached::{Cached, TimedSizedCache};
use graphql_client::GraphQLQuery;
use tokio::sync::Mutex;
use yew::{
    function_component, html, html::Scope, use_context, use_memo, Callback, Children, Component,
    ContextProvider, Html, Properties,
};
use yew_oauth2::prelude::OAuth2Context;

use crate::graphql::locations::get_location_details::GetLocationDetailsLocation;
use crate::graphql::locations::{get_location_details, GetLocationDetails};
use crate::graphql::sites::get_site_details::GetSiteDetailsSite;
use crate::graphql::sites::{list_sites, ListSites};
use crate::{
    error::FrontendError,
    graphql::query_with_credentials,
    graphql::sites::{get_site_details, GetSiteDetails},
};

#[derive(Clone, Debug, Default)]
pub struct SiteDetails {
    locations: Vec<u32>,
    name: String,
    address: Vec<String>,
}

impl SiteDetails {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn address(&self) -> &Vec<String> {
        &self.address
    }
    pub fn locations(&self) -> &Vec<u32> {
        &self.locations
    }
}

#[derive(Clone, Debug)]
pub struct ApiContext {
    auth_context: Option<OAuth2Context>,
    caches: Rc<Mutex<Caches>>,
}

#[derive(Debug)]
struct Caches {
    sites_cache: Rc<Mutex<Option<(Rc<Vec<u32>>, Instant)>>>,
    site_cache: TimedSizedCache<u32, Rc<Mutex<Option<Rc<SiteDetails>>>>>,
    location_cache: TimedSizedCache<u32, Rc<Mutex<Option<Rc<LocationDetails>>>>>,
}

impl Default for Caches {
    fn default() -> Self {
        Caches::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct LocationDetails {
    name: String,
}

impl LocationDetails {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Caches {
    fn new() -> Self {
        Self {
            sites_cache: Rc::new(Default::default()),
            site_cache: TimedSizedCache::with_size_and_lifespan(30, MAX_CACHE_TIME),
            location_cache: TimedSizedCache::with_size_and_lifespan(50, MAX_CACHE_TIME),
        }
    }
}

impl PartialEq for ApiContext {
    fn eq(&self, other: &Self) -> bool {
        self.auth_context == other.auth_context
    }
}

const MAX_CACHE_TIME: u64 = 90;

impl ApiContext {
    pub fn extract_from_scope<C: Component>(scope: &Scope<C>) -> Option<Rc<ApiContext>> {
        scope
            .context::<Rc<ApiContext>>(Callback::noop())
            .map(|t| t.0)
    }
    /// Send Graphql-Query to server
    pub async fn query<Q: GraphQLQuery>(
        &self,
        request: Q::Variables,
    ) -> Result<Q::ResponseData, FrontendError> {
        query_with_credentials::<Q>(&self.auth_context, request).await
    }

    pub async fn list_sites(&self) -> Result<Rc<Vec<u32>>, FrontendError> {
        let cache_mutex = self.caches.lock().await.sites_cache.clone();
        let mut cache = cache_mutex.lock().await;
        if let Some((found_result, poll_time)) = cache.deref() {
            if poll_time.elapsed().as_secs() < MAX_CACHE_TIME {
                return Ok(found_result.clone());
            }
        }
        let result = self.query::<ListSites>(list_sites::Variables {}).await?;
        let id_list = result
            .sites
            .into_iter()
            .map(|site_id| site_id.id.try_into())
            .collect::<Result<Vec<u32>, TryFromIntError>>()?;
        Ok(cache.insert((Rc::new(id_list), Instant::now())).0.clone())
    }

    pub async fn get_site_details(&self, id: u32) -> Result<Rc<SiteDetails>, FrontendError> {
        let entry_mutex = self
            .caches
            .lock()
            .await
            .site_cache
            .cache_get_or_set_with(id, Default::default)
            .clone();
        let mut entry_ref = entry_mutex.lock().await;
        if let Some(found_entry) = entry_ref.deref() {
            return Ok(found_entry.clone());
        }
        let data = Rc::new(
            self.query::<GetSiteDetails>(get_site_details::Variables { id: id.into() })
                .await?
                .site
                .map(
                    |GetSiteDetailsSite {
                         locations,
                         name,
                         address,
                     }| {
                        Ok::<SiteDetails, FrontendError>(SiteDetails {
                            locations: locations
                                .into_iter()
                                .map(|site_id| site_id.id.try_into())
                                .collect::<Result<Vec<u32>, TryFromIntError>>()?,
                            name,
                            address,
                        })
                    },
                )
                .transpose()?
                .unwrap_or_default(),
        );
        Ok(entry_ref.insert(data).clone())
    }
    pub async fn get_location_details(
        &self,
        id: u32,
    ) -> Result<Rc<LocationDetails>, FrontendError> {
        let entry_mutex = self
            .caches
            .lock()
            .await
            .location_cache
            .cache_get_or_set_with(id, Default::default)
            .clone();
        let mut entry_ref = entry_mutex.lock().await;
        if let Some(found_entry) = entry_ref.deref() {
            return Ok(found_entry.clone());
        }
        let data = Rc::new(
            self.query::<GetLocationDetails>(get_location_details::Variables { id: id.into() })
                .await?
                .location
                .map(|GetLocationDetailsLocation { name }| {
                    Ok::<LocationDetails, FrontendError>(LocationDetails { name })
                })
                .transpose()?
                .unwrap_or_default(),
        );
        Ok(entry_ref.insert(data).clone())
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ApiContextProvider)]
pub fn api_context(props: &ContextProps) -> Html {
    let auth_context = use_context::<OAuth2Context>();

    let context = use_memo(
        |_| ApiContext {
            auth_context,
            caches: Default::default(),
        },
        (),
    );
    html! {
        <ContextProvider<Rc<ApiContext>> {context}>
            {for props.children.iter()}
        </ContextProvider<Rc<ApiContext>>>
    }
}
