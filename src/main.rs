mod app;
mod components;
mod icons;
mod models;
//mod route;
mod services;
mod views;

use leptos::mount_to_body;
use crate::app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(App);
}
