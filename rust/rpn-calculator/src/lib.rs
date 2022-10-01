#[derive(Debug, PartialEq)]
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
        _ => panic!("Invalid Operation"),
    };
}

fn is_value(input: &CalculatorInput) -> Option<i32> {
    return match input {
        CalculatorInput::Value(x) => Some(*x),
        _ => None,
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
                let val_two = op_stack.pop();
                let val_one = op_stack.pop();

                if val_one == None || val_two == None {
                    return None;
                }

                let val_one = is_value(&val_one.unwrap());
                let val_two = is_value(&val_two.unwrap());

                op_stack.push(CalculatorInput::Value(perform_calculation(
                    &input,
                    val_one.unwrap(),
                    val_two.unwrap(),
                )))
            }
        }
    }

    if op_stack.len() != 1 {
        return None;
    }

    let potential_result = op_stack.pop().unwrap();
    return match potential_result {
        CalculatorInput::Value(val) => Some(val),
        _ => None,
    };
}
