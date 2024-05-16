use gpui::*;

use crate::theme::Theme;

pub enum IconType {
    Backspace,
    Close,
}

impl IconType {
    fn path(self) -> &'static str {
        match self {
            IconType::Backspace => "icons/backspace.svg",
            IconType::Close => "icons/x.svg",
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
        let mut element = svg().path(self.path).flex_none();

        if let Some(color) = self.color {
            element = element.text_color(color);
        } else {
            let theme = cx.global::<Theme>();
            element = element.text_color(theme.colors.text);
        }

        if let Some(size) = self.size {
            element = element.size(size);
        } else {
            element = element.size(rems(1.25));
        }

        element
    }
}

pub fn icon(icon: IconType) -> Icon {
    Icon {
        path: icon.path().into(),
        color: None,
        size: None,
    }
}
