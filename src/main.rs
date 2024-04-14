mod app;
mod components;
mod icons;
mod models;
mod route;
mod services;
mod views;

use crate::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
