use gpui::App;

use crate::state::{Event, Operator, StateEntity};

pub fn set_keybinds(app: &mut App) {
    // I don't really understand the idiomatic way to do this, so naturally I made a macro
    macro_rules! keybinds {
        ($( { keys: [$($key:expr),* $(,)?], action: $action:tt, event: $event:expr } ),* $(,)?) => {
            // create unit struct for the action
            gpui::actions!(
                calculator,
                [$($action,)*]
            );

            // register the action to emit the event from StateEntity
            $(app.on_action(|_: &$action, app| StateEntity::emit($event, app));)*

            // bind the keys to trigger the action
            app.bind_keys([$($(gpui::KeyBinding::new($key, $action, None),)*)*]);
        };
    }

    keybinds!(
        // inputs
        { keys: ["."], action: Decimal, event: Event::Input('.') },
        { keys: ["0"], action: Zero, event: Event::Input('0') },
        { keys: ["1"], action: One, event: Event::Input('1') },
        { keys: ["2"], action: Two, event: Event::Input('2') },
        { keys: ["3"], action: Three, event: Event::Input('3') },
        { keys: ["4"], action: Four, event: Event::Input('4') },
        { keys: ["5"], action: Five, event: Event::Input('5') },
        { keys: ["6"], action: Six, event: Event::Input('6') },
        { keys: ["7"], action: Seven, event: Event::Input('7') },
        { keys: ["8"], action: Eight, event: Event::Input('8') },
        { keys: ["9"], action: Nine, event: Event::Input('9') },

        // operator selection
        { keys: ["*"], action: Multiply, event: Event::Select(Operator::Multiply) },
        { keys: ["/"], action: Divide, event: Event::Select(Operator::Divide) },
        { keys: ["+"], action: Add, event: Event::Select(Operator::Add) },
        { keys: ["-"], action: Subtract, event: Event::Select(Operator::Subtract) },
        { keys: ["^"], action: Exponent, event: Event::Select(Operator::Exponent) },

        // direct operations
        { keys: ["backspace"], action: Backspace, event: Event::Backspace },
        { keys: ["ctrl-c"], action: Clear, event: Event::Clear },
        { keys: ["!"], action: Negate, event: Event::Negate },
        { keys: ["ctrl-1"], action: Inverse, event: Event::Inverse },
        { keys: ["ctrl-2"], action: Square, event: Event::Square },
        { keys: ["ctrl-3"], action: SquareRoot, event: Event::SquareRoot },
        { keys: ["ctrl-p"], action: Pi, event: Event::Pi },
        { keys: ["enter", "="], action: Equals, event: Event::Equals },
    );
}
