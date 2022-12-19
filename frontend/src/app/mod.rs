use lazy_static::lazy_static;
use log::error;
use patternfly_yew::BackdropViewer;
use patternfly_yew::Page;
use patternfly_yew::PageSidebar;
use patternfly_yew::ToastViewer;
use reqwest::Url;
use wasm_bindgen_futures::spawn_local;
use yew::html_nested;
use yew::{function_component, Context, Properties};
use yew::{html, Html};
use yew_nested_router::Router;
use yew_nested_router::Switch;
use yew_oauth2::oauth2::OAuth2;
use yew_oauth2::prelude::oauth2::Config;
use yew_oauth2::prelude::Authenticated;
use yew_oauth2::prelude::Failure;
use yew_oauth2::prelude::NotAuthenticated;

use route::AppRoute;

use crate::app::route::AuthenticatedSidebar;
use crate::app::route::NotAuthenticatedSidebar;
use crate::graphql::query;
use crate::graphql::settings::settings::{ResponseData, SettingsSettings};
use crate::graphql::settings::{settings, Settings};

pub mod route;

lazy_static! {
    static ref HOME_URL: Url = format!("{}/", crate::graphql::host()).parse().unwrap();
}

pub struct App {
    oauth2_config: Option<Config>,
}
#[derive(Properties, PartialEq)]
pub struct Props {
    pub config: Config,
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(config) = self.oauth2_config.clone() {
            html! {
                <MainOAuth2 {config}/>
            }
        } else {
            html! {
                <h1>{"Fetching"}</h1>
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
#[function_component(MainOAuth2)]
fn main_oauth2(props: &Props) -> Html {
    let oauth2_config = &props.config;
    html! {
        <OAuth2 config={oauth2_config.clone()}>
            <Router<AppRoute> default={AppRoute::Home}>
                <MainPage/>
            </Router<AppRoute>>
        </OAuth2>
    }
}

#[function_component(MainPage)]
fn main_page() -> Html {
    html! {
        <BackdropViewer>
            <ToastViewer>
                <Failure>{"Fail"}</Failure>
                <Authenticated>
                    <Page sidebar={html_nested! {<PageSidebar><AuthenticatedSidebar/></PageSidebar>}}>
                      //logo={logo}
                        <Switch<AppRoute>
                            render = {|r: AppRoute| r.main_content()}
                        />
                    </Page>
                </Authenticated>
                <NotAuthenticated>
                    <Page sidebar={html_nested! {<PageSidebar><NotAuthenticatedSidebar/></PageSidebar>}}>
                        <Switch<AppRoute>
                            render = {|r: AppRoute| r.unauthenticated_content()}
                        />
                    </Page>
                </NotAuthenticated>
            </ToastViewer>
        </BackdropViewer>
    }
}
fn switch_main(switch: AppRoute) -> Html {
    switch.main_content()
}
