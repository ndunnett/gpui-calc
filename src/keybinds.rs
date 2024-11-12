use gpui::*;

use crate::state::{Event, Operator, StateModel};

actions!(
    calculator,
    [
        Decimal, Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Backspace, Clear,
        Multiply, Divide, Add, Subtract, Exponent, Negate, Inverse, Square, SquareRoot, Pi, Equals
    ]
);

pub fn set_keybinds(cx: &mut AppContext) {
    // input actions
    cx.on_action(|_: &Decimal, cx| StateModel::emit(Event::Input('.'), cx));
    cx.on_action(|_: &Zero, cx| StateModel::emit(Event::Input('0'), cx));
    cx.on_action(|_: &One, cx| StateModel::emit(Event::Input('1'), cx));
    cx.on_action(|_: &Two, cx| StateModel::emit(Event::Input('2'), cx));
    cx.on_action(|_: &Three, cx| StateModel::emit(Event::Input('3'), cx));
    cx.on_action(|_: &Four, cx| StateModel::emit(Event::Input('4'), cx));
    cx.on_action(|_: &Five, cx| StateModel::emit(Event::Input('5'), cx));
    cx.on_action(|_: &Six, cx| StateModel::emit(Event::Input('6'), cx));
    cx.on_action(|_: &Seven, cx| StateModel::emit(Event::Input('7'), cx));
    cx.on_action(|_: &Eight, cx| StateModel::emit(Event::Input('8'), cx));
    cx.on_action(|_: &Nine, cx| StateModel::emit(Event::Input('9'), cx));

    // operation actions
    cx.on_action(|_: &Backspace, cx| StateModel::emit(Event::Backspace, cx));
    cx.on_action(|_: &Clear, cx| StateModel::emit(Event::Clear, cx));
    cx.on_action(|_: &Multiply, cx| StateModel::emit(Event::Select(Operator::Multiply), cx));
    cx.on_action(|_: &Divide, cx| StateModel::emit(Event::Select(Operator::Divide), cx));
    cx.on_action(|_: &Add, cx| StateModel::emit(Event::Select(Operator::Add), cx));
    cx.on_action(|_: &Subtract, cx| StateModel::emit(Event::Select(Operator::Subtract), cx));
    cx.on_action(|_: &Exponent, cx| StateModel::emit(Event::Select(Operator::Exponent), cx));
    cx.on_action(|_: &Negate, cx| StateModel::emit(Event::Negate, cx));
    cx.on_action(|_: &Inverse, cx| StateModel::emit(Event::Inverse, cx));
    cx.on_action(|_: &Square, cx| StateModel::emit(Event::Square, cx));
    cx.on_action(|_: &SquareRoot, cx| StateModel::emit(Event::SquareRoot, cx));
    cx.on_action(|_: &Pi, cx| StateModel::emit(Event::Pi, cx));
    cx.on_action(|_: &Equals, cx| StateModel::emit(Event::Equals, cx));

    // bindings
    cx.bind_keys([
        KeyBinding::new(".", Decimal, None),
        KeyBinding::new("0", Zero, None),
        KeyBinding::new("1", One, None),
        KeyBinding::new("2", Two, None),
        KeyBinding::new("3", Three, None),
        KeyBinding::new("4", Four, None),
        KeyBinding::new("5", Five, None),
        KeyBinding::new("6", Six, None),
        KeyBinding::new("7", Seven, None),
        KeyBinding::new("8", Eight, None),
        KeyBinding::new("9", Nine, None),
        KeyBinding::new("backspace", Backspace, None),
        KeyBinding::new("ctrl-c", Clear, None),
        KeyBinding::new("*", Multiply, None),
        KeyBinding::new("/", Divide, None),
        KeyBinding::new("+", Add, None),
        KeyBinding::new("-", Subtract, None),
        KeyBinding::new("^", Exponent, None),
        KeyBinding::new("!", Negate, None),
        KeyBinding::new("ctrl-1", Inverse, None),
        KeyBinding::new("ctrl-2", Square, None),
        KeyBinding::new("ctrl-3", SquareRoot, None),
        KeyBinding::new("ctrl-p", Pi, None),
        KeyBinding::new("=", Equals, None),
        KeyBinding::new("enter", Equals, None),
    ]);
}
