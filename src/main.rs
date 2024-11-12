mod app;
mod components;
mod icons;
mod models;
//mod route;
mod services;
mod views;

use leptos::*;
use crate::app::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
