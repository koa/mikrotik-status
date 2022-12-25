use std::rc::Rc;

use log::error;
use patternfly_yew::Card;
use patternfly_yew::DescriptionList;
use patternfly_yew::Spinner;
use web_sys::MouseEvent;
use yew::platform::spawn_local;
use yew::{html, Callback, Component, Context, Html, Properties};

use crate::components::context::{ApiContext, LocationDetails};

pub struct LocationCard {
    id: u32,
    data: DataState,
}
enum DataState {
    Loading,
    //NotFound,
    Data(Rc<LocationDetails>),
    Error,
}

pub enum LocationMsg {
    CardClicked,
    Loaded(Rc<LocationDetails>),
    Error,
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct LocationCardProps {
    pub id: u32,
}

impl Component for LocationCard {
    type Message = LocationMsg;
    type Properties = LocationCardProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id,
            data: DataState::Loading,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LocationMsg::CardClicked => false,
            LocationMsg::Loaded(data) => {
                self.data = DataState::Data(data);
                true
            }
            LocationMsg::Error => {
                self.data = DataState::Error;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.data {
            DataState::Loading => html! {<Spinner/>},
            DataState::Data(data) => {
                let onclick: Callback<MouseEvent> = ctx.link().callback(|e: MouseEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    LocationMsg::CardClicked
                });
                let title = format!("Raum {}", data.name());
                let title = html! {<>{title}</>};
                html! {
                    <Card {title} selectable=true {onclick}>
                        <DescriptionList>
                        </DescriptionList>
                    </Card>
                }
            }
            DataState::Error => html! {<p>{"Error"}</p>},
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
                        Ok(site) => {
                            scope.send_message(LocationMsg::Loaded(site));
                        }
                        Err(err) => {
                            scope.send_message(LocationMsg::Error);
                            error!("Error on server {err:?}");
                        }
                    }
                });
            } else {
                scope.send_message(LocationMsg::Error);
                error!("No Context found");
            }
        }
    }
}
