use gpui::*;
use prelude::FluentBuilder;

use crate::theme::Theme;

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>;

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    child: Option<AnyElement>,
    on_click: Option<ClickHandler>,
    bg: Option<Rgba>,
    hover: Option<Rgba>,
    active: Option<Rgba>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            child: Default::default(),
            on_click: Default::default(),
            bg: Default::default(),
            hover: Default::default(),
            active: Default::default(),
        }
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(listener));
        self
    }

    pub fn bg(mut self, color: Rgba) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn hover(mut self, color: Rgba) -> Self {
        self.hover = Some(color);
        self
    }

    pub fn active(mut self, color: Rgba) -> Self {
        self.active = Some(color);
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_element().into_any());
        self
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let bg = self
            .bg
            .unwrap_or_else(|| cx.global::<Theme>().colors.bg_button);

        let child = self.child.unwrap_or(div().into_any());

        div()
            .id(self.id)
            .w_full()
            .flex()
            .flex_grow()
            .items_center()
            .justify_center()
            .rounded_lg()
            .cursor_pointer()
            .child(child)
            .bg(bg)
            .when_some(self.hover, |this, cl| this.hover(|this| this.bg(cl)))
            .when_some(self.active, |this, cl| this.active(|this| this.bg(cl)))
            .when_some(self.on_click, |this, handler| this.on_click(handler))
    }
}

pub fn button(id: impl Into<ElementId>) -> Button {
    Button::new(id)
}
