use gpui::*;

pub enum WindowsButtonType {
    Minimize,
    Maximize,
    Restore,
    Close,
}

impl WindowsButtonType {
    pub fn code(self) -> &'static str {
        match self {
            WindowsButtonType::Minimize => "\u{e921}",
            WindowsButtonType::Maximize => "\u{e922}",
            WindowsButtonType::Restore => "\u{e923}",
            WindowsButtonType::Close => "\u{e8bb}",
        }
    }
}

pub fn windows_button(label: impl IntoElement, hover_color: Rgba) -> Div {
    div()
        .w(px(36.))
        .h_full()
        .flex()
        .items_center()
        .justify_center()
        .content_center()
        .font_family("Segoe Fluent Icons")
        .text_size(px(10.))
        .child(label)
        .hover(|style| style.bg(hover_color))
}
