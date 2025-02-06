use gpui::{div, prelude::*, rems, Window};

use crate::ui::Titlebar;

impl Render for Titlebar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().h(rems(1.5))
    }
}
