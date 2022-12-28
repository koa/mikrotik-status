use std::rc::Rc;

use log::error;
use patternfly_yew::Card;
use patternfly_yew::DescriptionGroup;
use patternfly_yew::DescriptionList;
use patternfly_yew::Spinner;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::{html, Callback, Component, Context, Html, Properties};
use yew_nested_router::prelude::RouterContext;

use crate::app::route::AppRoute;
use crate::components::context::{ApiContext, SiteDetails};

pub struct SiteCard {
    id: u32,
    data: DataState,
}
enum DataState {
    Loading,
    //NotFound,
    Data(Rc<SiteDetails>),
    Error,
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct SiteProperties {
    pub id: u32,
}

pub enum SiteMsg {
    CardClicked,
    Loaded(Rc<SiteDetails>),
    Error,
}

impl Component for SiteCard {
    type Message = SiteMsg;
    type Properties = SiteProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id,
            data: DataState::Loading,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            SiteMsg::CardClicked => {
                let navigator = ctx
                    .link()
                    .context::<RouterContext<AppRoute>>(Default::default())
                    .expect("Cannot be used outside of a router")
                    .0;
                navigator.push(AppRoute::site(self.id));
                false
            }

            SiteMsg::Loaded(data) => {
                self.data = DataState::Data(data);
                true
            }
            /*SiteMsg::Loaded(None) => {
                self.data = DataState::NotFound;
                true
            }*/
            SiteMsg::Error => {
                self.data = DataState::Error;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.data {
            DataState::Loading => html! {<Spinner/>},
            //DataState::NotFound => html! {<p>{"Not found"}</p>},
            DataState::Data(data) => {
                let title = html! {<>{data.name()}</>};
                let address = &data.address();

                let content = address
                    .iter()
                    .map(|line| html! {<p>{line}</p>})
                    .collect::<Html>();
                let onclick: Callback<MouseEvent> = ctx.link().callback(|e: MouseEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    SiteMsg::CardClicked
                });
                let count = data.locations().len();
                let mut properties = Vec::new();
                if count > 0 {
                    properties.push(html! {
                        <DescriptionGroup term="Räume">
                                {count}
                        </DescriptionGroup>
                    })
                }
                html! {
                    <Card {title} selectable=true {onclick}>
                        {content}
                        <DescriptionList>
                            {properties}
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
                    let result = api.get_site_details(id).await;
                    match result {
                        Ok(site) => {
                            scope.send_message(SiteMsg::Loaded(site));
                        }
                        Err(err) => {
                            scope.send_message(SiteMsg::Error);
                            error!("Error on server {err:?}");
                        }
                    }
                });
            } else {
                scope.send_message(SiteMsg::Error);
                error!("No Context found");
            }
        }
    }
}
