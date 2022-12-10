use std::rc::Rc;

use patternfly_yew::Card;
use patternfly_yew::DescriptionGroup;
use patternfly_yew::DescriptionList;
use web_sys::MouseEvent;
use yew::{html, Callback, Component, Context, Html, Properties};
use yew_router::agent::RouteRequest;
use yew_router::prelude::RouteAgentDispatcher;
use yew_router::RouterState;

use crate::app::route::AppRoute;
use crate::graphql::sites::list_sites::ListSitesSites;

pub struct SiteComponent<STATE: RouterState = ()> {
    router: RouteAgentDispatcher<STATE>,
    pub data: Rc<ListSitesSites>,
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct SiteProperties {
    pub data: Rc<ListSitesSites>,
}

pub enum SiteMsg {
    CardClicked,
    Nothing,
}

impl Component for SiteComponent {
    type Message = SiteMsg;
    type Properties = SiteProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            router: RouteAgentDispatcher::new(),
            data: ctx.props().data.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SiteMsg::CardClicked => {
                if let Ok(id) = self.data.id.try_into() {
                    self.router
                        .send(RouteRequest::ChangeRoute(AppRoute::site(id).into()));
                };
                false
            }
            SiteMsg::Nothing => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = html! {<>{self.data.name.as_str()}</>};
        let address = &self.data.address;

        let content = address
            .iter()
            .map(|line| html! {<p>{line}</p>})
            .collect::<Html>();
        let onclick: Callback<MouseEvent> = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            SiteMsg::CardClicked
        });
        let count = self.data.count_locations;
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
