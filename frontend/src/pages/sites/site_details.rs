use std::rc::Rc;

use log::error;
use yew::platform::spawn_local;
use yew::{html, Component, Context, Html, Properties};

use crate::components::context::ApiContext;
use crate::components::context::SiteDetails as Data;
use crate::components::location::LocationCard;

pub struct SiteDetailsPage {
    id: u32,
    details: Option<Rc<Data>>,
}

pub enum SiteDetailsMsg {
    UpdateSite(Rc<Data>),
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct SiteDetailsProps {
    pub id: u32,
}

impl Component for SiteDetailsPage {
    type Message = SiteDetailsMsg;
    type Properties = SiteDetailsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id,
            details: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SiteDetailsMsg::UpdateSite(data) => {
                self.details = Some(data);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let devices: Html = self
            .details
            .as_ref()
            .map(|d| {
                d.locations()
                    .iter()
                    .map(|id| html! {<LocationCard {id}/>})
                    .collect::<Html>()
            })
            .unwrap_or_default();
        let address = self.details.as_ref().map(|d| {
            d.address()
                .iter()
                .map(|line| html! {<p>{line}</p>})
                .collect::<Html>()
        });
        html! {
            <div class="pf-c-panel">
              <div class="pf-c-panel__header">{"Räume"}</div>
              if let Some(address) = address{
                <hr class="pf-c-divider" />
                <div class="pf-c-panel__main">
                  <div class="pf-c-panel__main-body">{address}</div>
                </div>
              }
              <hr class="pf-c-divider" />
              <div class="pf-c-panel__main">
                <div class="pf-c-panel__main-body card-grid">{devices}</div>
              </div>
            </div>
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let scope = ctx.link().clone();
            if let Some(api) = ApiContext::extract_from_scope(&scope) {
                let id = self.id;
                spawn_local(async move {
                    let result = api.get_site_details(id).await;
                    match result {
                        Ok(site_data) => {
                            scope.send_message(SiteDetailsMsg::UpdateSite(site_data));
                        }
                        Err(err) => error!("Error on server {err:?}"),
                    }
                });
            }
        }
    }
}

pub struct SiteDetailsHeader {
    id: u32,
    details: Option<Rc<Data>>,
}

pub enum SiteDetailsHeaderMsg {
    UpdateSite(Rc<Data>),
}

impl Component for SiteDetailsHeader {
    type Message = SiteDetailsHeaderMsg;
    type Properties = SiteDetailsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id,
            details: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SiteDetailsHeaderMsg::UpdateSite(data) => {
                self.details = Some(data);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(data) = &self.details {
            html!(format!("Standort {}", data.name()))
        } else {
            html!(format!("Standort {}", self.id))
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let scope = ctx.link().clone();
            if let Some(api) = ApiContext::extract_from_scope(&scope) {
                let id = self.id;
                spawn_local(async move {
                    let result = api.get_site_details(id).await;
                    match result {
                        Ok(site_data) => {
                            scope.send_message(SiteDetailsHeaderMsg::UpdateSite(site_data));
                        }
                        Err(err) => error!("Error on server {err:?}"),
                    }
                });
            }
        }
    }
}
