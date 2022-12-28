use std::rc::Rc;

use log::error;
use patternfly_yew::Spinner;
use yew::classes;
use yew::{html, platform::spawn_local, Component, Context, Html, Properties};

use crate::components::device::DeviceComponent;
use crate::components::{
    context::{ApiContext, LocationDetails},
    location::LocationCard,
};

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
                <>
                    <LocationCard id={self.id}/>
                    <div class={classes!("card-grid")}>{data.devices().iter().map(|id| html!{<DeviceComponent {id}/>}).collect::<Html>()}</div>
                </>
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
