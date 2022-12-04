use std::rc::Rc;

use itertools::Itertools;
use log::error;
use patternfly_yew::Card;
use patternfly_yew::Color;
use patternfly_yew::Label;
use wasm_bindgen_futures::spawn_local;
use yew::{html, Component, Context, Html, Properties};

use crate::error::FrontendError;
use crate::graphql::devices::list_devices::ListDevicesDevices;
use crate::graphql::devices::ping_device::PingDeviceDevice;
use crate::graphql::devices::ping_device::PingDeviceDevicePing;
use crate::graphql::devices::{ping_device, PingDevice};
use crate::graphql::query;

pub struct DeviceComponent {
    data: Rc<ListDevicesDevices>,
    ping_result: Option<PingDeviceDevicePing>,
    error: Option<String>,
}
#[derive(Clone, PartialEq, Eq, Properties)]
pub struct DeviceProperties {
    pub data: Rc<ListDevicesDevices>,
}

pub enum DeviceUpdateMessage {
    PingResult(PingDeviceDevicePing),
    PingError(String),
}

impl Component for DeviceComponent {
    type Message = DeviceUpdateMessage;
    type Properties = DeviceProperties;

    fn create(ctx: &Context<Self>) -> Self {
        DeviceComponent {
            data: ctx.props().data.clone(),
            ping_result: None,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DeviceUpdateMessage::PingResult(status) => {
                self.ping_result = Some(status);
                true
            }
            DeviceUpdateMessage::PingError(error_msg) => {
                self.error = Some(error_msg);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = html! {
            <>
                {self.data.name.as_str()}
            </>
        };
        let ping_result = if let Some(result) = self.ping_result.as_ref() {
            match result.answer.as_ref() {
                None => html!(<Label color={Color::Red} label="Failed"/>),
                Some(x) => {
                    html!(<Label color={Color::Green} label={format!("Success: {} ms", x.duration_in_ms)}/>)
                }
            }
        } else if let Some(error) = self.error.as_ref() {
            html!(<Label color={Color::Grey} label={error.clone()}/>)
        } else {
            html!(<Label label="pending"/>)
        };
        html! {
            <Card {title}>
                {ping_result}
            </Card>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let scope = ctx.link().clone();
            let id = self.data.id;
            spawn_local(async move {
                let result =
                    query::<PingDevice, _>(scope.clone(), ping_device::Variables { id }).await;
                match result {
                    Ok(ping_device::ResponseData {
                        device: Some(PingDeviceDevice { ping }),
                    }) => {
                        scope.send_message(DeviceUpdateMessage::PingResult(ping));
                    }
                    Ok(ping_device::ResponseData { device: None }) => error!("Empty ping answer"),
                    Err(FrontendError::Graphql(errors)) => {
                        scope.send_message(DeviceUpdateMessage::PingError(
                            errors.into_iter().map(|e| e.message).join("\n"),
                        ));
                    }
                    Err(err) => error!("Error on server {err:?}"),
                }
            });
        }
    }
}
