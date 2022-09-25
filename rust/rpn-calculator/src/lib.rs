#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn perform_calculation(operator: &CalculatorInput, val_one: i32, val_two: i32) -> i32 {
    return match operator {
        CalculatorInput::Add => val_one + val_two,
        CalculatorInput::Subtract => val_one - val_two,
        CalculatorInput::Multiply => val_one * val_two,
        CalculatorInput::Divide => val_one / val_two,
        _ => {
            panic!("Invalid Operation");
        }
    };
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut op_stack: Vec<CalculatorInput> = Vec::new();

    for input in inputs.iter() {
        match input {
            CalculatorInput::Value(x) => {
                op_stack.push(CalculatorInput::Value(*x));
            }
            _ => {
                let val_one = op_stack.pop().unwrap();
                let val_two = op_stack.pop().unwrap();

                match val_one {
                    CalculatorInput::Value(v1) => match val_two {
                        CalculatorInput::Value(v2) => {
                            op_stack
                                .push(CalculatorInput::Value(perform_calculation(&input, v1, v2)));
                        }
                        _ => {
                            panic!("Incorrect State")
                        }
                    },
                    _ => {
                        panic!("Incorrect State")
                    }
                }
            }
        }
    }

    let potential_result = op_stack.pop().unwrap();
    return match potential_result {
        CalculatorInput::Value(val) => Some(val),
        _ => None,
    };
}
