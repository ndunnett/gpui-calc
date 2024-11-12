#[derive(Clone, Copy, PartialEq)]
pub enum Operator {
    Constant,
    Exponent,
    Multiply,
    Divide,
    Add,
    Subtract,
}

#[derive(Clone, Copy)]
struct Operation {
    operator: Operator,
    operand: Option<f64>,
}

pub struct Calculator {
    operations: Vec<Operation>,
    selected: Option<Operator>,
    input: Option<String>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
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

    pub fn display(&self) -> String {
        if self.input.is_none() {
            self.formatted_calculation()
        } else {
            self.input.clone().unwrap()
        }
    }

    pub fn selected(&self) -> Option<Operator> {
        self.selected
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

    pub fn select(&mut self, operator: Operator) {
        if self.selected == Some(operator) {
            self.selected = None;
        } else {
            self.push_operation();
            self.selected = Some(operator);
        }
    }

    pub fn input(&mut self, c: char) {
        if self.input.is_none() {
            self.input = Some("".into());
        }

        if let Some(input) = &mut self.input {
            if c == '.' {
                if input.contains('.') {
                    return;
                } else if input.is_empty() {
                    input.push('0');
                }
            }

            input.push(c);
        }
    }

    pub fn backspace(&mut self) {
        if let Some(input) = &mut self.input {
            if !input.is_empty() {
                input.pop();
            }
        }
    }

    pub fn clear(&mut self) {
        self.operations.clear();
        self.selected = None;
        self.input = None;
    }

    pub fn negate(&mut self) {
        self.select(Operator::Multiply);
        self.input = Some("-1".into());
        self.push_operation();
    }

    pub fn equals(&mut self) {
        if self.selected.is_none() && !self.operations.is_empty() {
            if let Some(last_op) = self.operations.last() {
                self.operations.push(*last_op);
            }
        }
        self.push_operation();
    }
}
