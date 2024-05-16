use gpui::*;

use crate::theme::Theme;
use crate::ui::{display::Display, keypad::Keypad, titlebar::Titlebar};

pub struct Root {
    titlebar: View<Titlebar>,
    display: View<Display>,
    keypad: View<Keypad>,
}

impl Root {
    pub fn build(cx: &mut WindowContext) -> View<Root> {
        let titlebar = Titlebar::build(cx);
        let display = Display::build(cx);
        let keypad = Keypad::build(cx);

        cx.new_view(|_cx| Root {
            titlebar,
            display,
            keypad,
        })
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .size_full()
            .flex()
            .flex_col()
            .flex_grow()
            .bg(theme.colors.bg_window)
            .text_color(theme.colors.text)
            .font_family(theme.fonts.family.to_string())
            .child(self.titlebar.clone())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .px_2()
                    .pb_2()
                    .gap_2()
                    .child(self.display.clone())
                    .child(self.keypad.clone()),
            )
    }
}
