use gpui::*;

use crate::state::{Calculator, Operator};

#[derive(Clone, Copy)]
pub enum Event {
    Input(char),
    Select(Operator),
    Backspace,
    Clear,
    Negate,
    Inverse,
    Square,
    SquareRoot,
    Pi,
    Equals,
}

impl Event {
    pub fn handler(self, calculator: &mut Calculator) {
        match self {
            Event::Select(operator) => calculator.select(operator),
            Event::Input(c) => calculator.input(c),
            Event::Backspace => calculator.backspace(),
            Event::Clear => calculator.clear(),
            Event::Negate => calculator.negate(),
            Event::Inverse => calculator.inverse(),
            Event::Square => calculator.square(),
            Event::SquareRoot => calculator.square_root(),
            Event::Pi => calculator.pi(),
            Event::Equals => calculator.equals(),
        }
    }
}

impl EventEmitter<Event> for Calculator {}
