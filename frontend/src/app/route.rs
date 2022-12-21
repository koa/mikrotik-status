use patternfly_yew::Nav;
use patternfly_yew::NavItem;
use patternfly_yew::NavRouterItem;
use yew::Callback;
use yew::MouseEvent;
use yew::{function_component, html, Html};
use yew_nested_router::Target;
use yew_oauth2::agent::OAuth2Operations;
use yew_oauth2::oauth2::use_auth_agent;
use yew_oauth2::oauth2::LocationRedirect;

use crate::pages::devices::DeviceList;
use crate::pages::sites::site_details::SiteDetails;
use crate::pages::sites::site_list::SiteList;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    Sites,
    Site {
        id: u32,
    },
    #[default]
    Devices,
}

impl AppRoute {
    pub fn site(id: u32) -> AppRoute {
        AppRoute::Site { id }
    }

    pub fn main_content(&self) -> Html {
        match self {
            AppRoute::Devices => html! {<DeviceList/>},
            AppRoute::Sites => html! {<SiteList/>},
            AppRoute::Site { id } => html! {<SiteDetails id={*id}/>},
        }
    }
    pub fn unauthenticated_content(&self) -> Html {
        match self {
            _ => html!(<LocationRedirect logout_href="/" />),
        }
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
            <NavRouterItem<AppRoute> to={AppRoute::Sites}>{"Gebäude"}</NavRouterItem<AppRoute>>
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
