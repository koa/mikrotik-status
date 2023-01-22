use patternfly_yew::Nav;
use patternfly_yew::NavItem;
use patternfly_yew::NavRouterItem;
use yew::Callback;
use yew::MouseEvent;
use yew::{function_component, html, Html};
use yew_nested_router::Target;
use yew_oauth2::{
    agent::OAuth2Operations,
    oauth2::{use_auth_agent, LocationRedirect},
};

use crate::pages::{
    devices::DeviceList,
    sites::{
        device_details::DeviceDetailsPage,
        location_details::{LocationDetailsHeader, LocationDetailsPage},
        site_details::{SiteDetailsHeader, SiteDetailsPage},
        site_list::SiteListPage,
    },
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    Sites,
    Site {
        id: u32,
    },
    #[default]
    Devices,
    Device {
        id: u32,
    },
    Location {
        id: u32,
    },
}

impl AppRoute {
    pub fn site(id: u32) -> AppRoute {
        AppRoute::Site { id }
    }
    pub fn location(id: u32) -> AppRoute {
        AppRoute::Location { id }
    }
    pub fn device(id: u32) -> AppRoute {
        AppRoute::Device { id }
    }

    pub fn main_content(&self) -> Html {
        match self {
            AppRoute::Devices => html! {<DeviceList/>},
            AppRoute::Device { id } => html! {<DeviceDetailsPage id={*id}/>},
            AppRoute::Sites => html! {<SiteListPage/>},
            AppRoute::Site { id } => html! {<SiteDetailsPage id={*id}/>},
            AppRoute::Location { id } => html! {<LocationDetailsPage id={*id}/>},
        }
    }
    pub fn nav_title(&self) -> Html {
        match self {
            AppRoute::Sites => html!("Standorte"),
            AppRoute::Site { id } => html!(<SiteDetailsHeader id={*id}/>),
            AppRoute::Devices => html!("aktive Geräte"),
            AppRoute::Device { .. } => html!("Device"),
            AppRoute::Location { id } => html! {<LocationDetailsHeader id={*id}/>},
        }
    }
    pub fn unauthenticated_content(&self) -> Html {
        html!(<LocationRedirect logout_href="/" />)
    }
}

#[function_component(AuthenticatedSidebar)]
pub fn authenticated_sidebar() -> Html {
    let agent = use_auth_agent().expect("Requires OAuth2Context component in parent hierarchy");
    let logout = Callback::from(move |_: MouseEvent| {
        if let Err(err) = agent.logout() {
            log::warn!("Failed to logout: {err}");
        }
    });
    html! {
        <Nav>
            <NavRouterItem<AppRoute> to={AppRoute::Sites}>{"Standorte"}</NavRouterItem<AppRoute>>
            <NavRouterItem<AppRoute> to={AppRoute::Devices}>{"Geräte"}</NavRouterItem<AppRoute>>
            <span onclick={logout}><NavItem>{"Logout"}</NavItem></span>
        </Nav>
    }
}
#[function_component(NotAuthenticatedSidebar)]
pub fn not_authenticated_sidebar() -> Html {
    let agent = use_auth_agent().expect("Requires OAuth2Context component in parent hierarchy");
    let login = Callback::from(move |_: MouseEvent| {
        if let Err(err) = agent.start_login() {
            log::warn!("Failed to start login: {err}");
        }
    });
    html! {
        <Nav>
            <span onclick={login}><NavItem>{"Login"}</NavItem></span>
        </Nav>
    }
}
