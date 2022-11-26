use std::rc::Rc;

use log::error;
use patternfly_yew::Card;
use wasm_bindgen_futures::spawn_local;
use yew::{html, Component, Context, Html, Properties};

use crate::graphql::devices::list_devices::ListDevicesDevices;
use crate::graphql::devices::ping_device::PingDeviceDevice;
use crate::graphql::devices::{ping_device, PingDevice};
use crate::graphql::query;

pub struct DeviceComponent {
    data: Rc<ListDevicesDevices>,
    ping_result: Option<bool>,
}
#[derive(Clone, PartialEq, Properties)]
pub struct DeviceProperties {
    pub data: Rc<ListDevicesDevices>,
}

pub enum DeviceUpdateMessage {
    PingResult(bool),
}

impl Component for DeviceComponent {
    type Message = DeviceUpdateMessage;
    type Properties = DeviceProperties;

    fn create(ctx: &Context<Self>) -> Self {
        DeviceComponent {
            data: ctx.props().data.clone(),
            ping_result: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DeviceUpdateMessage::PingResult(status) => {
                self.ping_result = Some(status);
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
        let ping_result = match self.ping_result {
            None => "Pending",
            Some(true) => "Success",
            Some(false) => "Failed",
        };
        html! {
            <Card {title}>
                <h1>{ping_result}</h1>
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
                    Err(err) => error!("Error on server {err:?}"),
                }
            });
        }
    }
}
