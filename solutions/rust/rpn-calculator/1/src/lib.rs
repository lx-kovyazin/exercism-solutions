#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let do_op = |stack: &mut Vec<i32>, op: fn(i32, i32) -> i32| -> Option<()> {
        stack
            .pop()
            .and_then(|rhs| stack.pop().and_then(|lhs| Some((lhs, rhs))))
            .and_then(|ops| Some(stack.push(op(ops.0, ops.1))))
    };
    
    inputs
        .iter()
        .try_fold(&mut Vec::new(), |stack, i| {
            match i {
                CalculatorInput::Value(o) => Some(stack.push(*o)),
                CalculatorInput::Add => do_op(stack, |lhs, rhs| lhs + rhs),
                CalculatorInput::Subtract => do_op(stack, |lhs, rhs| lhs - rhs),
                CalculatorInput::Multiply => do_op(stack, |lhs, rhs| lhs * rhs),
                CalculatorInput::Divide => do_op(stack, |lhs, rhs| lhs / rhs),
            }
            .and(Some(stack))
        })
        .and_then(|stack| if stack.len() == 1 { stack.pop() } else { None })
}
