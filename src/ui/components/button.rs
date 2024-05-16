use gpui::*;

use crate::theme::Theme;

type Listener = Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>;

#[derive(IntoElement)]
pub struct Button {
    child: AnyElement,
    listener: Option<Listener>,
    bg_color: Option<Rgba>,
    bg_hover_color: Option<Rgba>,
}

#[allow(dead_code)]
impl Button {
    pub fn on_click(
        mut self,
        listener: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.listener = Some(Box::new(listener));
        self
    }

    pub fn bg(mut self, color: Rgba) -> Self {
        self.bg_color = Some(color);
        self
    }

    pub fn bg_hover(mut self, color: Rgba) -> Self {
        self.bg_hover_color = Some(color);
        self
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let mut element = div()
            .w_full()
            .flex()
            .flex_grow()
            .items_center()
            .justify_center()
            .rounded_lg()
            .cursor_pointer()
            .child(self.child);

        if let Some(bg_color) = self.bg_color {
            element = element.bg(bg_color);
        } else {
            element = element.bg(theme.colors.bg_button);
        }

        if let Some(bg_hover_color) = self.bg_hover_color {
            element = element.hover(|this| this.bg(bg_hover_color));
        } else {
            element = element.hover(|this| this.bg(theme.colors.bg_button_hover));
        }

        if let Some(listener) = self.listener {
            element = element.on_mouse_down(MouseButton::Left, listener);
        }

        element
    }
}

pub fn button(child: impl IntoElement) -> Button {
    Button {
        child: child.into_element().into_any(),
        listener: None,
        bg_color: None,
        bg_hover_color: None,
    }
}
