#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod icon;
mod state;
mod theme;
mod view;

use crate::app::run_app;

fn main() {
    run_app()
}
