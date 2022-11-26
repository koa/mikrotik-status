use log::Level;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::start_app;

use crate::app::App;

mod app;
mod components;
mod error;
mod graphql;
mod pages;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(Level::Info));
    start_app::<App>();
}
