use gpui::*;

#[allow(dead_code)]
pub enum IconName {
    Backspace,
    Close,
}

impl IconName {
    pub fn path(self) -> &'static str {
        match self {
            IconName::Backspace => "icons/backspace.svg",
            IconName::Close => "icons/x.svg",
        }
    }
}

#[derive(IntoElement)]
pub struct Icon {
    path: SharedString,
    color: Rgba,
    size: Rems,
}

impl Icon {
    pub fn new(icon: IconName, color: Rgba, size: Rems) -> Self {
        Self {
            path: icon.path().into(),
            color,
            size,
        }
    }
}

impl RenderOnce for Icon {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        svg()
            .size(self.size)
            .flex_none()
            .path(self.path)
            .text_color(self.color)
    }
}
