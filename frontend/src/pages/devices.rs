use std::rc::Rc;

use log::error;
use patternfly_yew::Spinner;
use wasm_bindgen_futures::spawn_local;
use yew::classes;
use yew::{html, Component, Context, Html};

use crate::components::device::DeviceComponent;
use crate::error::FrontendError;
use crate::graphql::devices::list_devices::ListDevicesDevices;
use crate::graphql::devices::{list_devices, ListDevices};
use crate::graphql::query_with_scope;

pub struct DeviceList {
    visible_devices: Vec<Rc<ListDevicesDevices>>,
}

pub enum DeviceListMessage {
    UpdateDevices(Vec<ListDevicesDevices>),
}

impl Component for DeviceList {
    type Message = DeviceListMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DeviceList {
            visible_devices: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DeviceListMessage::UpdateDevices(devices) => {
                self.visible_devices = devices.into_iter().map(Rc::new).collect();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.visible_devices.is_empty() {
            html! {
                <Spinner/>
            }
        } else {
            let device_cards = self
                .visible_devices
                .iter()
                .map(|data| {
                    let id: u32 = data.id.try_into()?;
                    Ok(html! {<DeviceComponent id={id}/>})
                })
                .collect::<Result<Html, FrontendError>>();
            match device_cards {
                Ok(device_cards) => html! {<div class={classes!("card-grid")}>{device_cards}</div>},
                Err(e) => html! {<p>{"Error"}</p>},
            }
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
                        scope.send_message(DeviceListMessage::UpdateDevices(devices));
                    }
                    Err(err) => error!("Error on server {err:?}"),
                }
            });
        }
    }
}
