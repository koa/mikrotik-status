use patternfly_yew::Spinner;
use yew::{html, Component, Context, Html, Properties};

pub struct DeviceDetailsPage {
    id: u32,
    data: DeviceData,
}

pub enum DeviceDetailsPageMsg {}
#[derive(Clone, PartialEq, Eq, Properties)]
pub struct DeviceDetailsProps {
    pub id: u32,
}
enum DeviceData {
    Empty,
}

impl Component for DeviceDetailsPage {
    type Message = DeviceDetailsPageMsg;
    type Properties = DeviceDetailsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id,
            data: DeviceData::Empty,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.data {
            DeviceData::Empty => html! {<Spinner/>},
        }
    }
}
