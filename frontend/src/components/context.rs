use std::ops::Deref;
use std::rc::Rc;

use cached::{Cached, TimedSizedCache};
use graphql_client::GraphQLQuery;
use log::info;
use tokio::sync::{Mutex, MutexGuard};
use yew::{
    function_component, html, html::Scope, use_context, use_memo, Callback, Children, Component,
    ContextProvider, Html, Properties,
};
use yew_oauth2::prelude::OAuth2Context;

use crate::{
    error::FrontendError,
    graphql::query_with_credentials,
    graphql::sites::{get_site_details, GetSiteDetails},
};

#[derive(Clone, Debug, Default)]
pub struct SiteDetails {
    devices: Vec<u32>,
    name: String,
    address: Vec<String>,
}

impl SiteDetails {
    pub fn devices(&self) -> &Vec<u32> {
        &self.devices
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn address(&self) -> &Vec<String> {
        &self.address
    }
}

#[derive(Clone, Debug)]
pub struct ApiContext {
    auth_context: Option<OAuth2Context>,
    caches: Rc<Mutex<Caches>>,
}

#[derive(Debug)]
struct Caches {
    site_cache: TimedSizedCache<u32, Rc<Mutex<Option<Rc<SiteDetails>>>>>,
}

impl Caches {}

impl PartialEq for ApiContext {
    fn eq(&self, other: &Self) -> bool {
        self.auth_context == other.auth_context
    }
}

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
    async fn site_details_entry<'a>(&self, id: u32) -> Rc<Mutex<Option<Rc<SiteDetails>>>> {
        self.caches
            .lock()
            .await
            .site_cache
            .cache_get_or_set_with(id, Default::default)
            .clone()
    }

    pub async fn get_site_details(&self, id: u32) -> Result<Rc<SiteDetails>, FrontendError> {
        let entry_mutex = self.site_details_entry(id).await;
        let mut entry_ref: MutexGuard<Option<Rc<SiteDetails>>> = entry_mutex.lock().await;
        if let Some(found_entry) = entry_ref.deref() {
            return Ok(found_entry.clone());
        }
        let result = self
            .query::<GetSiteDetails>(get_site_details::Variables { id: id.into() })
            .await?;
        let data = Rc::new(
            result
                .site
                .map(|site| {
                    let devices = site
                        .locations
                        .into_iter()
                        .filter_map(|site_id| site_id.id.try_into().ok())
                        .collect::<Vec<u32>>();
                    SiteDetails {
                        devices,
                        name: site.name,
                        address: site.address,
                    }
                })
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
            caches: Rc::new(Mutex::new(Caches {
                site_cache: TimedSizedCache::with_size_and_lifespan(30, 90),
            })),
        },
        (),
    );
    html! {
        <ContextProvider<Rc<ApiContext>> {context}>
            {for props.children.iter()}
        </ContextProvider<Rc<ApiContext>>>
    }
}
