use std::rc::Rc;

use log::error;
use yew::classes;
use yew::platform::spawn_local;
use yew::{html, Component, Context, Html, Properties};

use crate::components::context::ApiContext;
use crate::components::context::SiteDetails as Data;
use crate::components::location::LocationCard;
use crate::components::site::SiteCard;

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
        html! {
            <>
                <SiteCard id = {self.id}/>
                <div class={classes!("card-grid")}>
                    {devices}
                </div>
            </>
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
