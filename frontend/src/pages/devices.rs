use itertools::Itertools;
use std::rc::Rc;

use log::error;
use wasm_bindgen_futures::spawn_local;
use yew::classes;
use yew::{html, Component, Context, Html};

use crate::components::device::DeviceComponent;
use crate::graphql::devices::list_devices::ListDevicesDevices;
use crate::graphql::devices::{list_devices, ListDevices};
use crate::graphql::query;

pub struct DeviceList {
    visible_devices: Vec<Rc<ListDevicesDevices>>,
}

pub enum DeviceListMessage {
    UpdateDevices(Vec<ListDevicesDevices>),
}

impl Component for DeviceList {
    type Message = DeviceListMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        DeviceList {
            visible_devices: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DeviceListMessage::UpdateDevices(devices) => {
                self.visible_devices = devices.into_iter().map(Rc::new).collect();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.visible_devices.is_empty() {
            html! {
                <svg
                  class="pf-c-spinner"
                  role="progressbar"
                  viewBox="0 0 100 100"
                  aria-label="Loading List of devices...">
                  <circle class="pf-c-spinner__path" cx="50" cy="50" r="45" fill="none" />
                </svg>
            }
        } else {
            let device_cards = self
                .visible_devices
                .iter()
                .map(|data| {
                    html! {<DeviceComponent {data}/>}
                })
                .collect::<Html>();
            html! {<div class={classes!("card-grid")}>{device_cards}</div>}
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let scope = ctx.link().clone();
            spawn_local(async move {
                let result =
                    query::<ListDevices, _>(scope.clone(), list_devices::Variables {}).await;
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
