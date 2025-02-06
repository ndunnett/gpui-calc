#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod state;
mod ui;

fn main() {
    crate::app::run()
}
