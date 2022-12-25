use std::rc::Rc;

use log::error;
use patternfly_yew::Spinner;
use wasm_bindgen_futures::spawn_local;
use yew::classes;
use yew::{html, Component, Context, Html};

use crate::components::context::ApiContext;
use crate::components::site::SiteCard;

pub struct SiteListPage {
    visible_sites: Rc<Vec<u32>>,
}

pub enum SiteListMsg {
    UpdateSites(Rc<Vec<u32>>),
}

impl Component for SiteListPage {
    type Message = SiteListMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            visible_sites: Default::default(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SiteListMsg::UpdateSites(sites) => {
                self.visible_sites = sites;
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
                .map(|id| {
                    html! {<SiteCard {id}/>}
                })
                .collect::<Html>();
            html! {<div class={classes!("card-grid")}>{device_cards}</div>}
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let scope = ctx.link().clone();
            if let Some(api) = ApiContext::extract_from_scope(&scope) {
                spawn_local(async move {
                    let result = api.list_sites().await;
                    match result {
                        Ok(sites) => {
                            scope.send_message(SiteListMsg::UpdateSites(sites));
                        }
                        Err(err) => error!("Error on server {err:?}"),
                    }
                });
            }
        }
    }
}
