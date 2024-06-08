#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod assets;
mod state;
mod theme;
mod ui;

use crate::app::run_app;

fn main() {
    run_app()
}
