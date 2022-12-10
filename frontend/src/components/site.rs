use log::error;
use patternfly_yew::Card;
use patternfly_yew::DescriptionGroup;
use patternfly_yew::DescriptionList;
use patternfly_yew::Spinner;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::{html, Callback, Component, Context, Html, Properties};
use yew_router::agent::RouteRequest;
use yew_router::prelude::RouteAgentDispatcher;
use yew_router::RouterState;

use crate::app::route::AppRoute;
use crate::components::site::DataState::Loading;
use crate::graphql::query;
use crate::graphql::sites::get_site_details::GetSiteDetailsSite;
use crate::graphql::sites::{get_site_details, GetSiteDetails};

pub struct SiteComponent<STATE: RouterState = ()> {
    router: RouteAgentDispatcher<STATE>,
    id: u32,
    data: DataState,
}
enum DataState {
    Loading,
    NotFound,
    Data(GetSiteDetailsSite),
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct SiteProperties {
    pub id: u32,
}

pub enum SiteMsg {
    CardClicked,
    Loaded(Option<GetSiteDetailsSite>),
}

impl Component for SiteComponent {
    type Message = SiteMsg;
    type Properties = SiteProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            router: RouteAgentDispatcher::new(),
            id: ctx.props().id,
            data: Loading,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SiteMsg::CardClicked => {
                self.router
                    .send(RouteRequest::ChangeRoute(AppRoute::site(self.id).into()));
                false
            }

            SiteMsg::Loaded(Some(data)) => {
                self.data = DataState::Data(data);
                true
            }
            SiteMsg::Loaded(None) => {
                self.data = DataState::NotFound;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.data {
            Loading => html! {<Spinner/>},
            DataState::NotFound => html! {<p>{"Not found"}</p>},
            DataState::Data(data) => {
                let title = html! {<>{data.name.as_str()}</>};
                let address = &data.address;

                let content = address
                    .iter()
                    .map(|line| html! {<p>{line}</p>})
                    .collect::<Html>();
                let onclick: Callback<MouseEvent> = ctx.link().callback(|e: MouseEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    SiteMsg::CardClicked
                });
                let count = data.locations.len();
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
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let scope = ctx.link().clone();
            let id = self.id.into();
            spawn_local(async move {
                let result =
                    query::<GetSiteDetails, _>(scope.clone(), get_site_details::Variables { id })
                        .await;
                match result {
                    Ok(get_site_details::ResponseData { site }) => {
                        scope.send_message(SiteMsg::Loaded(site));
                    }
                    Err(err) => error!("Error on server {err:?}"),
                }
            });
        }
    }
}
