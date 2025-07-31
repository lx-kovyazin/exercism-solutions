#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    inputs
        .iter()
        .try_fold(Vec::new(), |mut stack, input| {
            let result = match input {
                &CalculatorInput::Value(value) => value,
                operation => {
                    let rhs = stack.pop()?;
                    let lhs = stack.pop()?;
                    match operation {
                        CalculatorInput::Add      => lhs + rhs,
                        CalculatorInput::Subtract => lhs - rhs,
                        CalculatorInput::Multiply => lhs * rhs,
                        CalculatorInput::Divide   => lhs / rhs,
                        _ => unreachable!("A value input is already processed.")
                    }
                }
            };
            stack.push(result);
            Some(stack)
        })
        .and_then(|mut stack| (stack.len() == 1).then(move|| stack.pop().unwrap()))
}
