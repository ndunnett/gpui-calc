use gpui::*;

use crate::state::StateModel;

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

        cx.observe(&model.inner, |_this, _model, cx| {
            cx.notify();
        })
        .detach();

        let state = model.inner.read(cx);
        let value = state.display();

        // surely there is a better way to do this
        // todo: figure out how to get available space for an element
        // todo: figure out how to calculate text width for given font/style/characters
        let rs = cx.rem_size();
        let w = cx.viewport_size().width - 2. * rems(0.5).to_pixels(rs);
        let h_to_fit = 1.77 * w / value.len() as f32;

        div()
            .h(rems(3.25))
            .flex()
            .items_center()
            .justify_end()
            .font_weight(FontWeight::EXTRA_LIGHT)
            .text_size(rems(3.).to_pixels(rs).min(h_to_fit))
            .child(value)
    }
}
