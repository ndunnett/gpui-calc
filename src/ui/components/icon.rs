use gpui::*;

use crate::theme::Theme;

pub enum IconType {
    Delete,
    Diff,
    Divide,
    Equal,
    Minus,
    Plus,
    X,
}

impl IconType {
    fn path(self) -> &'static str {
        match self {
            IconType::Delete => "icons/delete.svg",
            IconType::Diff => "icons/diff.svg",
            IconType::Divide => "icons/divide.svg",
            IconType::Equal => "icons/equal.svg",
            IconType::Minus => "icons/minus.svg",
            IconType::Plus => "icons/plus.svg",
            IconType::X => "icons/x.svg",
        }
    }
}

#[derive(IntoElement)]
pub struct Icon {
    path: SharedString,
    color: Option<Rgba>,
    size: Option<Rems>,
}

#[allow(dead_code)]
impl Icon {
    pub fn size(mut self, size: Rems) -> Self {
        self.size = Some(size);
        self
    }

    pub fn color(mut self, color: Rgba) -> Self {
        self.color = Some(color);
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let color = self
            .color
            .unwrap_or_else(|| cx.global::<Theme>().colors.text);

        let size = self.size.unwrap_or(rems(1.25));

        svg()
            .path(self.path)
            .flex_none()
            .text_color(color)
            .size(size)
    }
}

pub fn icon(icon: IconType) -> Icon {
    Icon {
        path: icon.path().into(),
        color: None,
        size: None,
    }
}
