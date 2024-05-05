#![windows_subsystem = "windows"]

mod app;
mod state;
mod theme;
mod view;

use gpui::App;

use crate::app::run_app;

fn main() {
    run_app(App::new())
}
