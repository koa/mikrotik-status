use lazy_static::lazy_static;
use log::error;
use patternfly_yew::BackdropViewer;
use patternfly_yew::Nav;
use patternfly_yew::NavItem;
use patternfly_yew::NavRouterItem;
use patternfly_yew::Page;
use patternfly_yew::PageSidebar;
use patternfly_yew::ToastViewer;
use reqwest::Url;
use wasm_bindgen_futures::spawn_local;
use yew::MouseEvent;
use yew::{html, html_nested, Html};
use yew::{Callback, Context};
use yew_oauth2::agent::OAuth2Operations;
use yew_oauth2::agent::{LogoutOptions, OAuth2Dispatcher};
use yew_oauth2::oauth2::Client;
use yew_oauth2::oauth2::LocationRedirect;
use yew_oauth2::oauth2::OAuth2;
use yew_oauth2::prelude::oauth2::Config;
use yew_oauth2::prelude::Authenticated;
use yew_oauth2::prelude::Failure;
use yew_oauth2::prelude::NotAuthenticated;
use yew_router::prelude::Switch;
use yew_router::router::{Render, Router};

use crate::graphql::query;
use crate::graphql::settings::settings::{ResponseData, SettingsSettings};
use crate::graphql::settings::{settings, Settings};
use crate::pages::devices::DeviceList;
use crate::pages::sites::SiteList;

#[derive(Switch, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[to = "/sites"]
    Sites,
    #[to = "/devices"]
    Devices,
    #[to = "/"]
    Home,
}

lazy_static! {
    static ref HOME_URL: Url = format!("{}/", crate::graphql::host()).parse().unwrap();
}

pub struct App {
    oauth2_config: Option<Config>,
}

impl App {
    fn switch_main() -> Render<AppRoute, ()> {
        Router::render(|switch| {
            Self::page(
                match switch {
                    AppRoute::Home => html! {<h1>{"Home"}</h1>},
                    AppRoute::Devices => html! {<DeviceList/>},
                    AppRoute::Sites => html! {<SiteList/>},
                },
                true,
            )
        })
    }
    fn switch_unauthenticated() -> Render<AppRoute, ()> {
        Router::render(|switch| match switch {
            AppRoute::Home => Self::page(html! {  <p> { "You need to log in" } </p>}, false),
            _ => html!(<LocationRedirect logout_href="/" />),
        })
    }
    fn page(html: Html, logged_in: bool) -> Html {
        let logout: Callback<MouseEvent> = Callback::from(|_: MouseEvent| {
            OAuth2Dispatcher::<Client>::new().logout_opts(LogoutOptions {
                target: Some(HOME_URL.clone()),
            });
        });
        let login: Callback<MouseEvent> = Callback::from(|_: MouseEvent| {
            OAuth2Dispatcher::<Client>::new().start_login();
        });
        let sidebar = if logged_in {
            html_nested! {
            <PageSidebar>
                <Nav>
                    <NavRouterItem<AppRoute> to={AppRoute::Home}>{"Start"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Sites}>{"Gebäude"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Devices}>{"Geräte"}</NavRouterItem<AppRoute>>
                    <span onclick={logout}><NavItem>{"Abmelden"}</NavItem></span>
                </Nav>
            </PageSidebar>
            }
        } else {
            html_nested! {
            <PageSidebar>
                <Nav>
                    <span onclick={login}><NavItem>{"Login"}</NavItem></span>
                </Nav>
            </PageSidebar>
            }
        };
        html! {
            <Page
                //logo={logo}
                sidebar={sidebar}
                >
                { html }
            </Page>
        }
    }
}

pub enum AppMessage {
    AuthenticationData(Config),
}

impl yew::Component for App {
    type Message = AppMessage;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            oauth2_config: None,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::AuthenticationData(config) => {
                self.oauth2_config = Some(config);
                true
            }
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        if let Some(oauth2_config) = self.oauth2_config.as_ref() {
            html! {
            <OAuth2 config={oauth2_config.clone()}>
                <Failure>{"Fail"}</Failure>
                <Authenticated>
                    <BackdropViewer/>
                    <ToastViewer/>

                    <Router<AppRoute, ()>
                        redirect = {Router::redirect(|_|AppRoute::Home)}
                        render = {Self::switch_main()}
                    />
                </Authenticated>
                <NotAuthenticated>
                    <Router<AppRoute, ()>
                        redirect = {Router::redirect(|_|AppRoute::Home)}
                        render = {Self::switch_unauthenticated()}
                    />

                </NotAuthenticated>

            </OAuth2>
            }
        } else {
            html! {
                <h1>{"Connecting to server"}</h1>
            }
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let scope = ctx.link().clone();
            spawn_local(async move {
                let result = query::<Settings, _>(scope.clone(), settings::Variables {}).await;
                match result {
                    Ok(ResponseData {
                        settings:
                            SettingsSettings {
                                auth_url,
                                client_id,
                                token_url,
                            },
                    }) => {
                        scope.send_message(AppMessage::AuthenticationData(Config {
                            client_id,
                            auth_url,
                            token_url,
                        }));
                    }
                    Err(err) => error!("Error on server {err:?}"),
                }
            });
        }
    }
}
