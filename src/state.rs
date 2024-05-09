use gpui::*;

#[derive(Clone, Copy)]
pub enum Operator {
    Constant,
    Exponent,
    Multiply,
    Divide,
    Add,
    Subtract,
}

#[derive(Clone, Copy)]
pub struct Operation {
    pub operator: Operator,
    pub operand: Option<f64>,
}

#[derive(Clone, Copy)]
pub enum Event {
    Input(char),
    Select(Operator),
    Backspace,
    Clear,
    Negate,
    Equals,
}

pub struct State {
    pub operations: Vec<Operation>,
    pub selected: Option<Operator>,
    pub input: Option<String>,
}

impl State {
    pub fn new() -> Self {
        State {
            operations: vec![],
            selected: None,
            input: None,
        }
    }

    pub fn calculation(&self) -> f64 {
        self.operations
            .iter()
            .fold(0_f64, |acc, op| match (op.operator, op.operand) {
                (Operator::Constant, Some(n)) => n,
                (Operator::Exponent, Some(n)) => acc.powf(n),
                (Operator::Multiply, Some(n)) => acc * n,
                (Operator::Divide, Some(n)) => acc / n,
                (Operator::Add, Some(n)) => acc + n,
                (Operator::Subtract, Some(n)) => acc - n,
                _ => acc,
            })
    }

    pub fn formatted_calculation(&self) -> String {
        let result = self.calculation();

        if result == 0. {
            "0".into()
        } else {
            // round to 20 sig figs and format to string
            let shift = 10_f64.powi(20 - result.abs().log10().ceil() as i32);
            format!("{}", (result * shift).round() / shift)
        }
    }

    fn push_operation(&mut self) {
        let input = if let Some(input) = self.input.clone() {
            input.parse::<f64>().ok()
        } else {
            None
        };

        if let Some(selected) = self.selected {
            self.operations.push(Operation {
                operator: selected,
                operand: input,
            });
        } else if input.is_some() {
            self.operations.push(Operation {
                operator: Operator::Constant,
                operand: input,
            });
        }

        self.selected = None;
        self.input = None;
    }

    fn select(&mut self, operator: Operator) {
        self.push_operation();
        self.selected = Some(operator);
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Select(operator) => self.select(*operator),
            Event::Input(c) => {
                if self.input.is_none() {
                    self.input = Some("".into());
                }

                if let Some(input) = &mut self.input {
                    if *c == '.' {
                        if input.contains('.') {
                            return;
                        } else if input.is_empty() {
                            input.extend(['0']);
                        }
                    }

                    input.extend([*c]);
                }
            }
            Event::Backspace => {
                if let Some(input) = &mut self.input {
                    if !input.is_empty() {
                        input.pop();
                    }
                }
            }
            Event::Clear => {
                self.operations.clear();
                self.selected = None;
                self.input = None;
            }
            Event::Negate => {
                self.select(Operator::Multiply);
                self.input = Some("-1".into());
                self.push_operation();
            }
            Event::Equals => {
                if self.selected.is_none() && !self.operations.is_empty() {
                    if let Some(last_op) = self.operations.last() {
                        self.operations.push(*last_op);
                    }
                }
                self.push_operation();
            }
        }
    }
}

impl EventEmitter<Event> for State {}

#[derive(Clone)]
pub struct StateModel {
    pub inner: Model<State>,
}

impl StateModel {
    pub fn init(cx: &mut WindowContext) {
        let model = cx.new_model(|_| State::new());

        cx.subscribe(&model, |model, event: &Event, cx| {
            cx.update_model(&model, |state, cx| {
                state.handle_event(event);
                cx.notify();
            });
        })
        .detach();

        cx.set_global::<StateModel>(Self { inner: model });
    }

    pub fn emit(event: Event, cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|model, cx| {
            cx.update_model(&model.inner, |_state, cx| {
                cx.emit(event);
            })
        });
    }
}

impl Global for StateModel {}
