use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::icon::{Icon, IconName};
use crate::state::{Operator, State};
use crate::theme::Theme;

#[derive(Clone, Copy)]
struct Titlebar;

impl Titlebar {
    fn build(cx: &mut WindowContext) -> View<Self> {
        println!("\npx: {}\n", (rems(2.) * cx.rem_size()).0);
        cx.new_view(|_cx| Self)
    }

    fn windows_button(label: String, hover_color: Rgba) -> Div {
        div()
            .w(px(36.))
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .content_center()
            .font_family("Segoe Fluent Icons")
            .text_size(px(10.))
            .child(label)
            .hover(|style| style.bg(hover_color))
    }
}

impl Render for Titlebar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if cfg!(target_os = "windows") {
            let theme = cx.global::<Theme>();

            div()
                .flex()
                .flex_row()
                .h((2. * cx.rem_size()).max(px(32.)))
                .mb_2()
                .justify_end()
                .children([
                    Self::windows_button("\u{e921}".into(), theme.colors.bg_button_hover),
                    if cx.is_maximized() {
                        Self::windows_button("\u{e923}".into(), theme.colors.bg_button_hover)
                    } else {
                        Self::windows_button("\u{e922}".into(), theme.colors.bg_button_hover)
                    },
                    Self::windows_button("\u{e8bb}".into(), theme.colors.bg_close_button_hover),
                ])
                .when(cx.is_maximized(), |this| this.mt_2())
        } else {
            div().h(rems(1.5))
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
        let h_to_fit = 1.65 * w / value.len() as f32;

        div()
            .flex()
            .justify_end()
            .px_2()
            .font_weight(FontWeight::THIN)
            .line_height(rems(3.25))
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
        child: impl IntoElement,
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
            .bg(theme.colors.bg_button)
            .hover(|this| this.bg(theme.colors.bg_button_hover))
            .cursor_pointer()
            .child(child)
            .on_mouse_down(MouseButton::Left, listener)
    }

    fn icon(name: IconName, theme: &Theme) -> Icon {
        Icon::new(name, theme.colors.text, rems(1.25))
    }

    fn button_input(input: char, theme: &Theme) -> Div {
        Self::button(input.to_string(), theme, move |_, cx| {
            cx.global_mut::<State>().add_input(input);
        })
    }

    fn button_clear(theme: &Theme) -> Div {
        Self::button("C", theme, move |_, cx| {
            cx.set_global(State::new());
        })
    }

    fn button_multiply(theme: &Theme) -> Div {
        Self::button(Self::icon(IconName::Close, theme), theme, move |_, cx| {
            cx.global_mut::<State>().select(Operator::Multiply);
        })
    }

    fn button_divide(theme: &Theme) -> Div {
        Self::button("/", theme, move |_, cx| {
            cx.global_mut::<State>().select(Operator::Divide);
        })
    }

    fn button_delete(theme: &Theme) -> Div {
        Self::button(
            Self::icon(IconName::Backspace, theme),
            theme,
            move |_, cx| {
                cx.global_mut::<State>().backspace();
            },
        )
    }

    fn button_exponent(theme: &Theme) -> Div {
        Self::button("^", theme, move |_, cx| {
            cx.global_mut::<State>().select(Operator::Exponent);
        })
    }

    fn button_add(theme: &Theme) -> Div {
        Self::button("+", theme, move |_, cx| {
            cx.global_mut::<State>().select(Operator::Add);
        })
    }

    fn button_subtract(theme: &Theme) -> Div {
        Self::button("-", theme, move |_, cx| {
            cx.global_mut::<State>().select(Operator::Subtract);
        })
    }

    fn button_negate(theme: &Theme) -> Div {
        Self::button("+/-", theme, move |_, cx| {
            cx.global_mut::<State>().negate();
        })
    }

    fn button_equals(theme: &Theme) -> Div {
        Self::button("=", theme, move |_, cx| {
            cx.global_mut::<State>().equals();
        })
    }
}

impl Render for Keypad {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div().flex().flex_grow().p_2().child(Self::col().children([
            Self::row().children([
                Self::button_clear(theme),
                Self::button_multiply(theme),
                Self::button_divide(theme),
                Self::button_delete(theme),
            ]),
            Self::row().children([
                Self::button_input('7', theme),
                Self::button_input('8', theme),
                Self::button_input('9', theme),
                Self::button_exponent(theme),
            ]),
            Self::row().children([
                Self::button_input('4', theme),
                Self::button_input('5', theme),
                Self::button_input('6', theme),
                Self::button_subtract(theme),
            ]),
            Self::row().children([
                Self::button_input('1', theme),
                Self::button_input('2', theme),
                Self::button_input('3', theme),
                Self::button_add(theme),
            ]),
            Self::row().children([
                Self::button_negate(theme),
                Self::button_input('0', theme),
                Self::button_input('.', theme),
                Self::button_equals(theme),
            ]),
        ]))
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
            .font_family(theme.fonts.family.to_string())
            .child(self.titlebar.clone())
            .child(self.display.clone())
            .child(self.keypad.clone())
    }
}
