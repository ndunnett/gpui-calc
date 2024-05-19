use gpui::*;

use crate::state::StateModel;
use crate::theme::Theme;

#[derive(Clone, Copy)]
pub struct Display;

impl Display {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for Display {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let model = cx.global::<StateModel>().clone();
        let theme = cx.global::<Theme>();
        let state = model.inner.read(cx);
        let value = state.display();

        // better than the previous solution but still unsure if
        // this is the best way to dynamically size text
        let rs = cx.rem_size();
        let available_width = cx.viewport_size().width - 2. * rems(0.5).to_pixels(rs);
        let text_system = cx.text_system();
        let font_id = text_system.resolve_font(&font(theme.fonts.family.clone()));
        let text_height = rems(3.).to_pixels(rs);
        let text_width = value.chars().fold(px(0.), |acc, ch| {
            acc + text_system
                .advance(font_id, text_height, ch)
                .unwrap_or_default()
                .width
        });

        cx.observe(&model.inner, |_this, _model, cx| {
            cx.notify();
        })
        .detach();

        div()
            .h(rems(3.25))
            .flex()
            .items_center()
            .justify_end()
            .font_weight(FontWeight::EXTRA_LIGHT)
            .text_size(text_height * (available_width / text_width).min(1.))
            .child(value)
    }
}
