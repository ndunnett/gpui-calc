use gpui::*;

use crate::state::{Event, Operator, StateModel};
use crate::ui::components::*;

fn col() -> Div {
    div().flex().flex_col().flex_grow().gap_1()
}

fn row() -> Div {
    div().flex().flex_row().flex_grow().gap_1()
}

fn button_emitting(label: impl IntoElement, event: Event) -> Button {
    button(label).on_click(move |_, cx| {
        StateModel::emit(event, cx);
    })
}

fn button_input(input: char) -> Button {
    button_emitting(input.to_string(), Event::Input(input))
}

#[derive(Clone, Copy)]
pub struct Keypad;

impl Keypad {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for Keypad {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        col().children([
            row().children([
                button_emitting("C", Event::Clear),
                button_emitting(icon(IconType::Close), Event::Select(Operator::Multiply)),
                button_emitting("/", Event::Select(Operator::Divide)),
                button_emitting(icon(IconType::Backspace), Event::Backspace),
            ]),
            row().children([
                button_input('7'),
                button_input('8'),
                button_input('9'),
                button_emitting("^", Event::Select(Operator::Exponent)),
            ]),
            row().children([
                button_input('4'),
                button_input('5'),
                button_input('6'),
                button_emitting("-", Event::Select(Operator::Subtract)),
            ]),
            row().children([
                button_input('1'),
                button_input('2'),
                button_input('3'),
                button_emitting("+", Event::Select(Operator::Add)),
            ]),
            row().children([
                button_emitting("+/-", Event::Negate),
                button_input('0'),
                button_input('.'),
                button_emitting("=", Event::Equals),
            ]),
        ])
    }
}
