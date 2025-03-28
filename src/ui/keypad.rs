use gpui::{div, prelude::*, rgba, App, Div, ElementId, Entity, Rems, Window};

use crate::{
    state::{Event, Operator, StateEntity},
    ui::components::{button, icon, Button, IconType},
};

fn col() -> Div {
    div().flex().flex_col().flex_grow().gap_1()
}

fn row() -> Div {
    div().flex().flex_row().flex_grow().gap_1()
}

fn button_emitting(id: impl Into<ElementId>, label: impl IntoElement, event: Event) -> Button {
    button(id)
        .child(label)
        .on_click(move |_, _, app| {
            StateEntity::emit(event, app);
        })
        .hover(rgba(0x55555555))
        .active(rgba(0x35353555))
}

fn button_select(
    id: impl Into<ElementId>,
    label: impl IntoElement,
    cx: &mut Context<Keypad>,
    operator: Operator,
) -> Button {
    let state = cx.global::<StateEntity>().inner.read(cx);

    button_emitting(id, label, Event::Select(operator)).when_some(
        state.selected(),
        |this, selected| {
            this.when(operator == selected, |this| {
                this.bg(rgba(0x515151AA))
                    .hover(rgba(0x595959AA))
                    .active(rgba(0x494949AA))
            })
        },
    )
}

fn button_input(id: impl Into<ElementId>, input: char) -> Button {
    button_emitting(id, input.to_string(), Event::Input(input))
}

#[derive(Clone, Copy)]
pub struct Keypad;

impl Keypad {
    pub fn build(app: &mut App) -> Entity<Self> {
        app.new(|_cx| Self)
    }
}

impl Render for Keypad {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        col().children([
            row().children([
                button_emitting("inverse", "¹/𝑥", Event::Inverse),
                button_emitting("pi", icon(IconType::Pi).size(Rems(1.)), Event::Pi),
                button_emitting("clear", "C", Event::Clear),
                button_emitting("backspace", icon(IconType::Delete), Event::Backspace),
            ]),
            row().children([
                button_emitting("square_root", icon(IconType::Radical), Event::SquareRoot),
                button_emitting("square", "𝑥 ²", Event::Square),
                button_select("exponent", "𝑥 ʸ", cx, Operator::Exponent),
                button_select("divide", icon(IconType::Divide), cx, Operator::Divide),
            ]),
            row().children([
                button_input("seven", '7'),
                button_input("eight", '8'),
                button_input("nine", '9'),
                button_select("multiply", icon(IconType::X), cx, Operator::Multiply),
            ]),
            row().children([
                button_input("four", '4'),
                button_input("five", '5'),
                button_input("six", '6'),
                button_select("subtract", icon(IconType::Minus), cx, Operator::Subtract),
            ]),
            row().children([
                button_input("one", '1'),
                button_input("two", '2'),
                button_input("three", '3'),
                button_select("add", icon(IconType::Plus), cx, Operator::Add),
            ]),
            row().children([
                button_emitting("negate", icon(IconType::Diff).size(Rems(1.)), Event::Negate),
                button_input("zero", '0'),
                button_input("decimal", '.'),
                button_emitting("equals", icon(IconType::Equal), Event::Equals),
            ]),
        ])
    }
}
