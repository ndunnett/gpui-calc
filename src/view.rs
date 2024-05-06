use gpui::*;

use crate::state::{Operator, State};
use crate::theme::Theme;

#[derive(Clone, Copy)]
struct Titlebar;

impl Titlebar {
    fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }

    fn windows_button<F>(label: String, hover_color: F) -> Div
    where
        F: Into<Fill>,
    {
        div()
            .w(rems(2.25))
            .h(rems(1.9))
            .flex()
            .items_center()
            .justify_center()
            .child(label)
            .hover(|this| this.bg(hover_color))
    }
}

impl Render for Titlebar {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        if cfg!(target_os = "windows") {
            div()
                .flex()
                .flex_row()
                .justify_end()
                .child(Self::windows_button("_".into(), rgba(0xaaaaaa33)))
                .child(Self::windows_button("□".into(), rgba(0xaaaaaa33)))
                .child(Self::windows_button("×".into(), rgb(0xcc2222)))
        } else {
            div().h(rems(1.))
        }
    }
}

#[derive(Clone, Copy)]
struct Display;

impl Display {
    fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for Display {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let state = cx.global::<State>();

        let value = if state.input.is_none() {
            state.formatted_calculation()
        } else {
            state.input.clone().unwrap()
        };

        // surely there is a better way to this
        // todo: figure out how to get available space for an element
        // todo: figure out how to calculate text width for given font/style/characters
        let rs = cx.rem_size();
        let w = cx.viewport_size().width - 2. * rems(0.5).to_pixels(rs);
        let h_to_fit = 1.82 * w / value.len() as f32;

        div()
            .flex()
            .justify_end()
            .font_weight(FontWeight::THIN)
            .line_height(rems(3.75))
            .text_size(rems(3.).to_pixels(rs).min(h_to_fit))
            .child(value)
    }
}

#[derive(Clone, Copy)]
struct Keypad;

impl Keypad {
    fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }

    fn col() -> Div {
        div().flex().flex_col().flex_grow().gap_1()
    }

    fn row() -> Div {
        div().flex().flex_row().flex_grow().gap_1()
    }

    fn button(
        label: String,
        theme: &Theme,
        listener: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Div {
        div()
            .w_full()
            .flex()
            .flex_grow()
            .items_center()
            .justify_center()
            .rounded_lg()
            .bg(theme.colors.bg_secondary)
            .hover(|this| this.bg(theme.colors.bg_primary))
            .cursor_pointer()
            .child(label)
            .on_mouse_down(MouseButton::Left, listener)
    }
}

impl Render for Keypad {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        Self::col()
            .child(
                Self::row()
                    .child(Self::button("C".into(), theme, move |_, cx| {
                        cx.set_global(State::new());
                    }))
                    .child(Self::button("*".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().select(Operator::Multiply);
                    }))
                    .child(Self::button("/".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().select(Operator::Divide);
                    }))
                    .child(Self::button("⌫".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().backspace();
                    })),
            )
            .child(
                Self::row()
                    .child(Self::button("7".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().add_input('7');
                    }))
                    .child(Self::button("8".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().add_input('8');
                    }))
                    .child(Self::button("9".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().add_input('9');
                    }))
                    .child(Self::button("^".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().select(Operator::Exponent);
                    })),
            )
            .child(
                Self::row()
                    .child(Self::button("4".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().add_input('4');
                    }))
                    .child(Self::button("5".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().add_input('5');
                    }))
                    .child(Self::button("6".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().add_input('6');
                    }))
                    .child(Self::button("-".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().select(Operator::Subtract);
                    })),
            )
            .child(
                Self::row()
                    .child(Self::button("1".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().add_input('1');
                    }))
                    .child(Self::button("2".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().add_input('2');
                    }))
                    .child(Self::button("3".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().add_input('3');
                    }))
                    .child(Self::button("+".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().select(Operator::Add);
                    })),
            )
            .child(
                Self::row()
                    .child(Self::button("+/-".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().negate();
                    }))
                    .child(Self::button("0".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().add_input('0');
                    }))
                    .child(Self::button(".".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().add_input('.');
                    }))
                    .child(Self::button("=".into(), theme, move |_, cx| {
                        cx.global_mut::<State>().equals();
                    })),
            )
    }
}

pub struct Root {
    titlebar: View<Titlebar>,
    display: View<Display>,
    keypad: View<Keypad>,
}

impl Root {
    pub fn build(cx: &mut WindowContext) -> View<Root> {
        cx.on_window_should_close(|cx| {
            cx.quit();
            true
        });

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
            .bg(theme.colors.bg_window)
            .text_color(theme.colors.text)
            .child(self.titlebar.clone())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .p_2()
                    .child(self.display.clone())
                    .child(self.keypad.clone()),
            )
    }
}
