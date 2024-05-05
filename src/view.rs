use gpui::*;

use crate::state::{Operator, State};
use crate::theme::Theme;

fn button(label: String, theme: &Theme) -> Div {
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

        let rs = cx.rem_size();
        let w = cx.viewport_size().width - 2. * rems(0.5).to_pixels(rs);
        let h_to_fit = 1.82 * w / value.len() as f32;

        div()
            .w(w)
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
}

impl Render for Keypad {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .flex_col()
            .flex_grow()
            .gap_1()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_grow()
                    .gap_1()
                    .child(button("C".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.set_global(State::new());
                        },
                    ))
                    .child(button("*".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().select(Operator::Multiply);
                        },
                    ))
                    .child(button("/".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().select(Operator::Divide);
                        },
                    ))
                    .child(button("⌫".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().backspace();
                        },
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_grow()
                    .gap_1()
                    .child(button("7".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().add_input('7');
                        },
                    ))
                    .child(button("8".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().add_input('8');
                        },
                    ))
                    .child(button("9".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().add_input('9');
                        },
                    ))
                    .child(button("^".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().select(Operator::Exponent);
                        },
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_grow()
                    .gap_1()
                    .child(button("4".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().add_input('4');
                        },
                    ))
                    .child(button("5".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().add_input('5');
                        },
                    ))
                    .child(button("6".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().add_input('6');
                        },
                    ))
                    .child(button("-".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().select(Operator::Subtract);
                        },
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_grow()
                    .gap_1()
                    .child(button("1".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().add_input('1');
                        },
                    ))
                    .child(button("2".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().add_input('2');
                        },
                    ))
                    .child(button("3".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().add_input('3');
                        },
                    ))
                    .child(button("+".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().select(Operator::Add);
                        },
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_grow()
                    .gap_1()
                    .child(button("+/-".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().negate();
                        },
                    ))
                    .child(button("0".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().add_input('0');
                        },
                    ))
                    .child(button(".".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().add_input('.');
                        },
                    ))
                    .child(button("=".into(), theme).on_mouse_down(
                        MouseButton::Left,
                        move |_, cx| {
                            cx.global_mut::<State>().equals();
                        },
                    )),
            )
    }
}

pub struct Root {
    display: View<Display>,
    keypad: View<Keypad>,
}

impl Root {
    pub fn build(cx: &mut WindowContext) -> View<Root> {
        cx.on_window_should_close(|cx| {
            cx.quit();
            true
        });

        let display = Display::build(cx);
        let keypad = Keypad::build(cx);
        cx.new_view(|_cx| Root { display, keypad })
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .size_full()
            .flex()
            .bg(theme.colors.bg_window)
            .text_color(theme.colors.text)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .mt_4()
                    .p_2()
                    .child(self.display.clone())
                    .child(self.keypad.clone()),
            )
    }
}
