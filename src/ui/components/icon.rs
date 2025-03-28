use gpui::{prelude::*, rems, svg, App, Rems, Rgba, SharedString, Window};

use crate::ui::Theme;

pub enum IconType {
    Delete,
    Diff,
    Divide,
    Equal,
    Minus,
    Pi,
    Plus,
    Radical,
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
            IconType::Pi => "icons/pi.svg",
            IconType::Plus => "icons/plus.svg",
            IconType::Radical => "icons/radical.svg",
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
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
