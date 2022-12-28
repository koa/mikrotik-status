use std::hash::Hash;
use std::num::TryFromIntError;
use std::ops::{Deref, DerefMut};
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

use crate::graphql::devices::get_device_details::GetDeviceDetailsDevice;
use crate::graphql::devices::{get_device_details, GetDeviceDetails};
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
pub struct DeviceDetails {
    name: String,
    has_routeros: bool,
}

impl DeviceDetails {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn has_routeros(&self) -> bool {
        self.has_routeros
    }
}

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
    device_cache: TimedSizedCache<u32, Rc<Mutex<Option<Rc<DeviceDetails>>>>>,
}

impl Default for Caches {
    fn default() -> Self {
        Caches::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct LocationDetails {
    name: String,
    devices: Vec<u32>,
}

impl LocationDetails {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn devices(&self) -> &Vec<u32> {
        &self.devices
    }
}

impl Caches {
    fn new() -> Self {
        Self {
            sites_cache: Rc::new(Default::default()),
            site_cache: TimedSizedCache::with_size_and_lifespan(30, MAX_CACHE_TIME),
            location_cache: TimedSizedCache::with_size_and_lifespan(50, MAX_CACHE_TIME),
            device_cache: TimedSizedCache::with_size_and_lifespan(50, MAX_CACHE_TIME),
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

    async fn fetch_cached_entry<Q, R, ID, CS, F, RE>(
        &self,
        id: ID,
        cache_selector: CS,
        fetcher: F,
        response_extractor: RE,
    ) -> Result<Rc<R>, FrontendError>
    where
        CS: Fn(&mut Caches) -> &mut TimedSizedCache<ID, Rc<Mutex<Option<Rc<R>>>>>,
        R: Default,
        ID: Hash + Eq + Clone,
        Q: GraphQLQuery,
        F: Fn(&ID) -> Q::Variables,
        RE: Fn(Q::ResponseData) -> Option<Result<R, FrontendError>>,
    {
        let entry_mutex = cache_selector(self.caches.lock().await.deref_mut())
            .cache_get_or_set_with(id.clone(), Default::default)
            .clone();
        let mut entry_ref = entry_mutex.lock().await;
        if let Some(found_entry) = entry_ref.deref() {
            return Ok(found_entry.clone());
        }
        let variables = fetcher(&id);
        let result = self.query::<Q>(variables).await?;
        let data = response_extractor(result);
        let ret = data.transpose()?.unwrap_or_default();
        Ok(entry_ref.insert(Rc::new(ret)).clone())
    }

    pub async fn get_site_details(&self, id: u32) -> Result<Rc<SiteDetails>, FrontendError> {
        self.fetch_cached_entry::<GetSiteDetails, _, _, _, _, _>(
            id,
            |c| &mut c.site_cache,
            |id| get_site_details::Variables { id: (*id).into() },
            |data: get_site_details::ResponseData| {
                data.site.map(
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
            },
        )
        .await
    }
    pub async fn get_location_details(
        &self,
        id: u32,
    ) -> Result<Rc<LocationDetails>, FrontendError> {
        self.fetch_cached_entry::<GetLocationDetails, _, _, _, _, _>(
            id,
            |c| &mut c.location_cache,
            |id| get_location_details::Variables { id: (*id).into() },
            |data| {
                data.location
                    .map(|GetLocationDetailsLocation { name, devices }| {
                        Ok::<LocationDetails, FrontendError>(LocationDetails {
                            name,
                            devices: devices
                                .iter()
                                .map(|d| d.id.try_into())
                                .collect::<Result<Vec<u32>, _>>()?,
                        })
                    })
            },
        )
        .await
    }
    pub async fn get_device_details(&self, id: u32) -> Result<Rc<DeviceDetails>, FrontendError> {
        self.fetch_cached_entry::<GetDeviceDetails, _, _, _, _, _>(
            id,
            |c| &mut c.device_cache,
            |id| get_device_details::Variables { id: (*id).into() },
            |data| {
                data.device.map(
                    |GetDeviceDetailsDevice {
                         id,
                         name,
                         location,
                         has_routeros,
                     }| {
                        Ok::<DeviceDetails, FrontendError>(DeviceDetails {
                            name,
                            has_routeros: has_routeros,
                        })
                    },
                )
            },
        )
        .await
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
