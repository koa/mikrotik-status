use yew::{html, Component, Context, Html, Properties};

pub struct SiteDetails {
    id: u32,
}

pub enum SiteDetailsMsg {}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct SiteDetailsProps {
    pub id: u32,
}

impl Component for SiteDetails {
    type Message = SiteDetailsMsg;
    type Properties = SiteDetailsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { id: ctx.props().id }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {<p>{format!("Site Details {}",self.id)}</p>}
    }
}
