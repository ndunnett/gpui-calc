use gpui::{div, prelude::*, App, Entity, Window};

use crate::ui::{Display, Keypad, Theme, Titlebar};

pub struct Root {
    titlebar: Entity<Titlebar>,
    display: Entity<Display>,
    keypad: Entity<Keypad>,
}

impl Root {
    pub fn build(window: &mut Window, app: &mut App) -> Entity<Root> {
        window.on_window_should_close(app, |_window: &mut Window, app: &mut App| {
            app.quit();
            true
        });

        let titlebar = Titlebar::build(app);
        let display = Display::build(app);
        let keypad = Keypad::build(app);

        app.new(|_cx| Root {
            titlebar,
            display,
            keypad,
        })
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
