use std::rc::Rc;

use log::error;
use patternfly_yew::Spinner;
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            visible_sites: vec![],
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SiteListMsg::UpdateSites(sites) => {
                self.visible_sites = sites.into_iter().map(Rc::new).collect();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.visible_sites.is_empty() {
            html! {
                <Spinner/>
            }
        } else {
            let device_cards = self
                .visible_sites
                .iter()
                .map(|data| data.id)
                .flat_map(|id| id.try_into())
                .map(|id: u32| {
                    html! {<SiteComponent {id}/>}
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
