use std::rc::Rc;

use log::error;
use patternfly_yew::Spinner;
use yew::{html, platform::spawn_local, Component, Context, Html, Properties};

use crate::components::context::SiteDetails;
use crate::components::device::DeviceComponent;
use crate::components::{
    context::{ApiContext, LocationDetails},
    location::LocationCard,
};
use crate::error::FrontendError;

pub struct LocationDetailsPage {
    id: u32,
    data: LocationData,
}
enum LocationData {
    Empty,
    Data(Rc<LocationDetails>),
}
pub enum LocationDetailsPageMsg {
    UpdateSite(Rc<LocationDetails>),
}
#[derive(Clone, PartialEq, Eq, Properties)]
pub struct LocationDetailsProps {
    pub id: u32,
}

impl Component for LocationDetailsPage {
    type Message = LocationDetailsPageMsg;
    type Properties = LocationDetailsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id,
            data: LocationData::Empty,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LocationDetailsPageMsg::UpdateSite(data) => {
                self.data = LocationData::Data(data);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.data {
            LocationData::Empty => html! {
                <>
                    <LocationCard id={self.id}/>
                    <Spinner/>
                </>
            },
            LocationData::Data(data) => html! {
                <div class="pf-c-panel">
                  <div class="pf-c-panel__header">{"Geräte"}</div>
                  <hr class="pf-c-divider" />
                  <div class="pf-c-panel__main">
                    <div class="pf-c-panel__main-body card-grid">{data.devices().iter().map(|id| html!{<DeviceComponent {id}/>}).collect::<Html>()}</div>
                  </div>
                </div>
            },
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let scope = ctx.link().clone();
            if let Some(api) = ApiContext::extract_from_scope(&scope) {
                let id = self.id;
                spawn_local(async move {
                    let result = api.get_location_details(id).await;
                    match result {
                        Ok(location_data) => {
                            scope.send_message(LocationDetailsPageMsg::UpdateSite(location_data));
                        }
                        Err(err) => error!("Error on server {err:?}"),
                    }
                });
            }
        }
    }
}

pub struct LocationDetailsHeader {
    id: u32,
    data: Option<(Rc<LocationDetails>, Option<Rc<SiteDetails>>)>,
}

pub enum LocationDetailsHeaderMsg {
    UpdateSite(Rc<LocationDetails>, Option<Rc<SiteDetails>>),
}

impl Component for LocationDetailsHeader {
    type Message = LocationDetailsHeaderMsg;
    type Properties = LocationDetailsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id,
            data: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LocationDetailsHeaderMsg::UpdateSite(location, site) => {
                self.data = Some((location, site));
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.data {
            None => html! {
                html!(format!("Raum {}", self.id))
            },
            Some((location, Some(site))) => {
                html!(format!(
                    "Standort {}, Raum {}",
                    site.name(),
                    location.name()
                ))
            }
            Some((location, None)) => html!(format!("Raum {}", location.name())),
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let scope = ctx.link().clone();
            if let Some(api) = ApiContext::extract_from_scope(&scope) {
                let id = self.id;
                spawn_local(async move {
                    let result = fetch_location_and_site(api, id).await;
                    match result {
                        Ok((location_data, site_data)) => {
                            scope.send_message(LocationDetailsHeaderMsg::UpdateSite(
                                location_data,
                                site_data,
                            ));
                        }
                        Err(err) => error!("Error on server {err:?}"),
                    }
                });
            }
        }
    }
}

async fn fetch_location_and_site(
    api: Rc<ApiContext>,
    location_id: u32,
) -> Result<(Rc<LocationDetails>, Option<Rc<SiteDetails>>), FrontendError> {
    let location = api.get_location_details(location_id).await?;
    let site = if let Some(site_id) = location.site() {
        Some(api.get_site_details(site_id).await?)
    } else {
        None
    };
    Ok((location, site))
}
