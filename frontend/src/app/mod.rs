use lazy_static::lazy_static;
use log::error;
use patternfly_yew::{BackdropViewer, Page, PageSidebar, ToastViewer};
use reqwest::Url;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, html_nested, Context, Html, Properties};
use yew_nested_router::prelude::use_router;
use yew_nested_router::{Router, Switch};
use yew_oauth2::{
    oauth2::OAuth2,
    prelude::{oauth2::Config, Authenticated, Failure, NotAuthenticated},
};

use route::{AppRoute, AuthenticatedSidebar, NotAuthenticatedSidebar};

use crate::{
    components::context::ApiContextProvider,
    graphql::{
        query_with_scope,
        settings::{
            settings::{self, ResponseData, SettingsSettings},
            Settings,
        },
    },
};

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
                let result =
                    query_with_scope::<Settings, _>(scope.clone(), settings::Variables {}).await;
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
            <Router<AppRoute> default={AppRoute::default()}>
                <MainPage/>
            </Router<AppRoute>>
        </OAuth2>
    }
}

#[function_component(MainPage)]
fn main_page() -> Html {
    let route = use_router::<AppRoute>().and_then(|router| router.active_target);
    let nav = route.as_ref().map(|r| r.nav_title()).unwrap_or_default();
    html! {
        <BackdropViewer>
            <ToastViewer>
                <Failure>{"Fail"}</Failure>
                <Authenticated>
                    <ApiContextProvider>
                        <Page sidebar={html_nested! {<PageSidebar><AuthenticatedSidebar/></PageSidebar>}} {nav}>
                          //logo={logo}
                            <Switch<AppRoute>
                                render = {|r: AppRoute| r.main_content()}
                            />
                        </Page>
                    </ApiContextProvider>
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
