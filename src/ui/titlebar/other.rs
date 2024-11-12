use gpui::*;

#[derive(Clone, Copy)]
pub struct Titlebar;

impl Titlebar {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for Titlebar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().h(rems(1.5))
    }
}
