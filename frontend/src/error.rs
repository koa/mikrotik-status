use std::error::Error;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::mem::discriminant;
use std::num::TryFromIntError;

use itertools::Itertools;
use log::error;
use reqwest::header::InvalidHeaderValue;
use thiserror::Error;
use wasm_bindgen::JsValue;
use yew::virtual_dom::VNode;
use yew::{html, Html};

pub struct JavascriptError {
    original_value: JsValue,
}

impl JavascriptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(string) = self.original_value.as_string() {
            f.write_str(&string)?;
        }
        Ok(())
    }
}

impl From<JsValue> for JavascriptError {
    fn from(value: JsValue) -> Self {
        JavascriptError {
            original_value: value,
        }
    }
}

impl Debug for JavascriptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Self::fmt(self, f)
    }
}

impl Display for JavascriptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Self::fmt(self, f)
    }
}

impl Error for JavascriptError {}

#[derive(Error, Debug)]
pub enum FrontendError {
    #[error("Generic Javascript error")]
    JS(#[from] JavascriptError),
    #[error("Cannot convert json")]
    Serde(#[from] serde_json::Error),
    #[error("Graphql Execution Error")]
    Graphql(Vec<graphql_client::Error>),
    #[error("Error on http request")]
    Reqwest(#[from] reqwest::Error),
    #[error("Invalid http header")]
    InvalidHeader(#[from] InvalidHeaderValue),
    #[error("Cannot convert int value")]
    TryFromInt(#[from] TryFromIntError),
}

impl FrontendError {
    pub fn format_error(&self) -> Html {
        if let FrontendError::Graphql(errors) = self {
            errors.iter().map(format_graphql_error).collect()
        } else {
            html! {<ol class="pf-c-list">{format_error(Some(self))}</ol>}
        }
    }
}

impl PartialEq for FrontendError {
    fn eq(&self, other: &Self) -> bool {
        if discriminant(self) == discriminant(other) {
            self.to_string() == other.to_string()
        } else {
            false
        }
    }
}

fn format_graphql_error(error: &graphql_client::Error) -> VNode {
    let path = error.path.as_ref().map(|path| path.iter().join("/"));
    html! {
        <dl class="pf-c-description-list pf-m-horizontal pf-m-fluid pf-m-2-col">
            if let Some(path)=path{
                <dt class="pf-c-description-list__term">{"Pfad"}</dt>
                <dt class="pf-c-description-list__description">{path}</dt>
            }
            <dt class="pf-c-description-list__term">{"Fehler"}</dt>
            <dl class="pf-c-description-list__description">{&error.message}</dl>
        </dl>
    }
}
fn format_error(error: Option<&dyn StdError>) -> Html {
    match error {
        None => html! {},
        Some(error) => html! {
        <>
            <li>{error.to_string()}</li>
            {format_error(error.source())}
        </>
        },
    }
}
