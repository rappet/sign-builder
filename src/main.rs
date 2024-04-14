mod app;
mod components;
mod icons;
mod models;
mod route;
mod services;
mod views;

use crate::app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
