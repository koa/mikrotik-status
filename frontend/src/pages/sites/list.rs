use std::rc::Rc;

use log::error;
use wasm_bindgen_futures::spawn_local;
use yew::classes;
use yew::{html, Component, Context, Html};

use crate::components::site::SiteComponent;
use crate::graphql::query;
use crate::graphql::sites::list_sites::{ListSitesSites, Variables};
use crate::graphql::sites::{list_sites, ListSites};

pub struct SiteList {
    visible_sites: Vec<Rc<ListSitesSites>>,
}

pub enum SiteListMsg {
    UpdateSites(Vec<ListSitesSites>),
}

impl Component for SiteList {
    type Message = SiteListMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            visible_sites: vec![],
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SiteListMsg::UpdateSites(sites) => {
                self.visible_sites = sites.into_iter().map(Rc::new).collect();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.visible_sites.is_empty() {
            html! {
                <svg
                  class="pf-c-spinner"
                  role="progressbar"
                  viewBox="0 0 100 100"
                  aria-label="Loading List of devices...">
                  <circle class="pf-c-spinner__path" cx="50" cy="50" r="45" fill="none" />
                </svg>
            }
        } else {
            let device_cards = self
                .visible_sites
                .iter()
                .map(|data| {
                    html! {<SiteComponent {data}/>}
                })
                .collect::<Html>();
            html! {<div class={classes!("card-grid")}>{device_cards}</div>}
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let scope = ctx.link().clone();
            spawn_local(async move {
                let result = query::<ListSites, _>(scope.clone(), Variables {}).await;
                match result {
                    Ok(list_sites::ResponseData { sites }) => {
                        scope.send_message(SiteListMsg::UpdateSites(sites));
                    }
                    Err(err) => error!("Error on server {err:?}"),
                }
            });
        }
    }
}
