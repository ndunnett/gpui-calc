use gpui::*;

use crate::state::StateModel;
use root::Root;

mod components;
mod display;
mod keypad;
mod root;
mod titlebar;

pub fn build_ui(cx: &mut WindowContext) -> View<Root> {
    cx.on_window_should_close(|cx| {
        cx.quit();
        true
    });

    StateModel::init(cx);
    Root::build(cx)
}
