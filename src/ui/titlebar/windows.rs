use gpui::{Div, Rgba, Window, WindowControlArea, div, prelude::*, px};

use crate::ui::{Theme, Titlebar};

impl Render for Titlebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let hover_color = theme.colors.bg_button_hover;
        let close_color = theme.colors.bg_close_button_hover;

        div()
            .flex()
            .flex_row()
            .h(window.rem_size() * if window.is_maximized() { 2.1 } else { 2. })
            .justify_end()
            .children([
                drag_space(),
                WindowsButton::Minimize.render(hover_color),
                if window.is_maximized() {
                    WindowsButton::Restore.render(hover_color)
                } else {
                    WindowsButton::Maximize.render(hover_color)
                },
                WindowsButton::Close.render(close_color),
            ])
    }
}

fn drag_space() -> Div {
    div()
        .flex_grow()
        .h_full()
        .window_control_area(WindowControlArea::Drag)
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

    fn control(&self) -> WindowControlArea {
        match self {
            WindowsButton::Minimize => WindowControlArea::Min,
            WindowsButton::Maximize => WindowControlArea::Max,
            WindowsButton::Restore => WindowControlArea::Max,
            WindowsButton::Close => WindowControlArea::Close,
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
            .window_control_area(self.control())
    }
}
