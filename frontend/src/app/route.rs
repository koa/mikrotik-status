use patternfly_yew::NavRouterItem;
use yew::{html, html_nested, Html};
use yew_router::Switch;

use crate::pages::devices::DeviceList;
use crate::pages::sites::list::SiteList;

#[derive(Switch, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[to = "/sites/list"]
    Sites,
    #[to = "/sites/{id}"]
    Site { id: u32 },
    #[to = "/devices"]
    Devices,
    #[to = "/"]
    Home,
}

impl Default for AppRoute {
    fn default() -> Self {
        AppRoute::Home
    }
}

impl AppRoute {
    pub fn site(id: u32) -> AppRoute {
        AppRoute::Site { id }
    }
    pub fn main_menu() -> Html {
        html_nested! {
            <>
                <NavRouterItem<AppRoute> to={AppRoute::Home}>{"Start"}</NavRouterItem<AppRoute>>
                <NavRouterItem<AppRoute> to={AppRoute::Sites}>{"Gebäude"}</NavRouterItem<AppRoute>>
                <NavRouterItem<AppRoute> to={AppRoute::Devices}>{"Geräte"}</NavRouterItem<AppRoute>>
            </>
        }
    }
    pub fn main_content(&self) -> Html {
        match self {
            AppRoute::Home => html! {<h1>{"Home"}</h1>},
            AppRoute::Devices => html! {<DeviceList/>},
            AppRoute::Sites => html! {<SiteList/>},
            AppRoute::Site { id } => html! {<p>{id}</p>},
        }
    }
}
