use std::rc::Rc;

use itertools::Itertools;
use log::error;
use patternfly_yew::Card;
use patternfly_yew::Color;
use patternfly_yew::Label;
use patternfly_yew::Spinner;
use wasm_bindgen_futures::spawn_local;
use yew::{html, Component, Context, Html, Properties};

use crate::components::context::{ApiContext, DeviceDetails};
use crate::{
    error::FrontendError,
    graphql::{
        devices::{
            ping_device::{self, PingDeviceDevice, PingDeviceDevicePing},
            PingDevice,
        },
        query_with_scope,
    },
};

pub struct DeviceComponent {
    id: u32,
    data: DataState,
    ping_result: PingState,
}
enum DataState {
    Loading,
    //NotFound,
    Data(Rc<DeviceDetails>),
    Error,
}
enum PingState {
    Invalid,
    Loading,
    Data(PingDeviceDevicePing),
    Error(String),
}
#[derive(Clone, PartialEq, Eq, Properties)]
pub struct DeviceProperties {
    pub id: u32,
}

pub enum DeviceUpdateMessage {
    QueryResult(Rc<DeviceDetails>),
    QueryError,
    PingResult(PingDeviceDevicePing),
    PingError(String),
}

impl Component for DeviceComponent {
    type Message = DeviceUpdateMessage;
    type Properties = DeviceProperties;

    fn create(ctx: &Context<Self>) -> Self {
        DeviceComponent {
            id: ctx.props().id,
            data: DataState::Loading,
            ping_result: PingState::Invalid,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DeviceUpdateMessage::PingResult(status) => {
                self.ping_result = PingState::Data(status);
                true
            }
            DeviceUpdateMessage::PingError(error_msg) => {
                self.ping_result = PingState::Error(error_msg);
                true
            }
            DeviceUpdateMessage::QueryResult(data) => {
                self.data = DataState::Data(data);
                true
            }
            DeviceUpdateMessage::QueryError => {
                self.data = DataState::Error;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.data {
            DataState::Loading => html! {<Spinner/>},
            DataState::Data(data) => {
                let title = html! {
                    <>
                        {data.name()}
                    </>
                };
                let ping_result = match &self.ping_result {
                    PingState::Invalid => html!(),
                    PingState::Loading => html!(<Label label="pending"/>),
                    PingState::Data(result) => match result.answer.as_ref() {
                        None => html!(<Label color={Color::Red} label="Failed"/>),
                        Some(x) => {
                            html!(<Label color={Color::Green} label={format!("Success: {} ms", x.duration_in_ms)}/>)
                        }
                    },
                    PingState::Error(error) => {
                        html!(<Label color={Color::Grey} label={error.clone()}/>)
                    }
                };

                html! {
                    <Card {title}>
                        {data.model_name().unwrap_or_default()}
                        {ping_result}
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
                    let Ok(device) = api.get_device_details(id).await else{
                      scope.send_message(DeviceUpdateMessage::QueryError);
                        return;
                    };
                    let has_routeros = device.has_routeros();
                    scope.send_message(DeviceUpdateMessage::QueryResult(device));
                    if !has_routeros {
                        return;
                    }
                    match query_with_scope::<PingDevice, _>(
                        scope.clone(),
                        ping_device::Variables { id: id.into() },
                    )
                    .await
                    {
                        Ok(ping_device::ResponseData {
                            device: Some(PingDeviceDevice { ping }),
                        }) => {
                            scope.send_message(DeviceUpdateMessage::PingResult(ping));
                        }
                        Ok(ping_device::ResponseData { device: None }) => {
                            error!("Empty ping answer")
                        }
                        Err(FrontendError::Graphql(errors)) => {
                            scope.send_message(DeviceUpdateMessage::PingError(
                                errors.into_iter().map(|e| e.message).join("\n"),
                            ));
                        }
                        Err(err) => error!("Error on server {err:?}"),
                    }
                })
            };
        }
    }
}
