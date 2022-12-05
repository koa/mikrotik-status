use std::rc::Rc;

use patternfly_yew::Card;
use yew::{html, Component, Context, Html, Properties};

use crate::graphql::sites::list_sites::ListSitesSites;

pub struct SiteComponent {
    pub data: Rc<ListSitesSites>,
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct SiteProperties {
    pub data: Rc<ListSitesSites>,
}

pub enum SiteMsg {}

impl Component for SiteComponent {
    type Message = SiteMsg;
    type Properties = SiteProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            data: ctx.props().data.clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = html! {
            <>
                {self.data.name.as_str()}
            </>
        };
        let address = &self.data.address;
        let content = address
            .iter()
            .map(|line| html! {<p>{line}</p>})
            .collect::<Html>();
        html! {
            <Card {title}>
                {content}
            </Card>
        }
    }
}
