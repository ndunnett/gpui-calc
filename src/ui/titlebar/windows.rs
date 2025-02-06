use gpui::{div, prelude::*, px, Div, Rgba, Window};

use crate::ui::{Theme, Titlebar};

impl Render for Titlebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .flex_row()
            .h(window.rem_size() * if window.is_maximized() { 2.1 } else { 2. })
            .justify_end()
            .children([
                WindowsButton::Minimize.render(theme.colors.bg_button_hover),
                if window.is_maximized() {
                    WindowsButton::Restore.render(theme.colors.bg_button_hover)
                } else {
                    WindowsButton::Maximize.render(theme.colors.bg_button_hover)
                },
                WindowsButton::Close.render(theme.colors.bg_close_button_hover),
            ])
    }
}

enum WindowsButton {
    Minimize,
    Maximize,
    Restore,
    Close,
}

impl WindowsButton {
    fn code(&self) -> &'static str {
        match self {
            WindowsButton::Minimize => "\u{e921}",
            WindowsButton::Maximize => "\u{e922}",
            WindowsButton::Restore => "\u{e923}",
            WindowsButton::Close => "\u{e8bb}",
        }
    }

    pub fn render(&self, hover_color: Rgba) -> Div {
        div()
            .w(px(36.))
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .content_center()
            .font_family("Segoe Fluent Icons")
            .text_size(px(10.))
            .child(self.code())
            .hover(|style| style.bg(hover_color))
    }
}
