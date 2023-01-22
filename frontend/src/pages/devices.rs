use std::rc::Rc;

use log::info;
use patternfly_yew::Spinner;
use wasm_bindgen_futures::spawn_local;
use yew::classes;
use yew::{html, Component, Context, Html};

use crate::components::device::DeviceComponent;
use crate::components::error::Error;
use crate::error::FrontendError;
use crate::graphql::devices::list_devices::ListDevicesDevices;
use crate::graphql::devices::{list_devices, ListDevices};
use crate::graphql::query_with_scope;

pub struct DeviceList {
    data: Data,
}

enum Data {
    Loading,
    Devices(Vec<Rc<ListDevicesDevices>>),
    Error(Rc<FrontendError>),
}

pub enum DeviceListMessage {
    UpdateDevices(Vec<ListDevicesDevices>),
    ReceivedError(FrontendError),
}

impl Component for DeviceList {
    type Message = DeviceListMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DeviceList {
            data: Data::Loading,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DeviceListMessage::UpdateDevices(devices) => {
                self.data = Data::Devices(devices.into_iter().map(Rc::new).collect());
                true
            }
            DeviceListMessage::ReceivedError(error) => {
                self.data = Data::Error(Rc::new(error));
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.data {
            Data::Loading => html! {<Spinner/>},
            Data::Devices(devices) => {
                let device_cards = devices
                    .iter()
                    .map(|data| {
                        let id: u32 = data.id.try_into()?;
                        Ok(html! {<DeviceComponent id={id}/>})
                    })
                    .collect::<Result<Html, FrontendError>>();
                match device_cards {
                    Ok(device_cards) => {
                        html! {<div class={classes!("card-grid")}>{device_cards}</div>}
                    }
                    Err(e) => html! {<p>{"Error"}</p>},
                }
            }
            Data::Error(error) => html! {<Error {error}/>},
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let scope = ctx.link().clone();
            spawn_local(async move {
                let result =
                    query_with_scope::<ListDevices, _>(scope.clone(), list_devices::Variables {})
                        .await;
                match result {
                    Ok(list_devices::ResponseData { devices }) => {
                        scope.send_message(DeviceListMessage::UpdateDevices(devices))
                    }
                    Err(err) => scope.send_message(DeviceListMessage::ReceivedError(err)),
                }
            });
        }
    }
}
