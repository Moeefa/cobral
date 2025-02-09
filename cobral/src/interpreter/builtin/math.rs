use crate::{
  interpreter::{error::InterpreterError, value::Value, EvalFn},
  shared::ast::{Expression, Location},
};

pub fn square_root(
  args: Vec<Expression>,
  location: Location,
  eval: EvalFn,
) -> Result<Value, InterpreterError> {
  let arg = args.get(0).ok_or(InterpreterError::ArgumentMismatchError(
    location,
    "square_root requires one argument".to_string(),
  ))?;

  match eval(arg.clone())? {
    Value::Float(f) => Ok(Value::Float(f.sqrt())),
    Value::Integer(i) => Ok(Value::Float((i as f64).sqrt())),
    _ => Err(InterpreterError::TypeError(
      location,
      "Expected number for square_root".to_string(),
    )),
  }
}

pub fn power(
  args: Vec<Expression>,
  location: Location,
  eval: EvalFn,
) -> Result<Value, InterpreterError> {
  if args.len() != 2 {
    return Err(InterpreterError::ArgumentMismatchError(
      location,
      "power requires two arguments".to_string(),
    ));
  }

  let base = eval(args[0].clone())?;
  let exponent = eval(args[1].clone())?;

  match (base, exponent) {
    (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1.powf(f2))),
    (Value::Float(f), Value::Integer(i)) => Ok(Value::Float(f.powi(i as i32))),
    (Value::Integer(i), Value::Float(f)) => Ok(Value::Float((i as f64).powf(f))),
    (Value::Integer(i1), Value::Integer(i2)) => Ok(Value::Integer(i1.pow(i2 as u32))),
    _ => Err(InterpreterError::TypeError(
      location,
      "Expected numbers for power".to_string(),
    )),
  }
}

pub fn pi(
  args: Vec<Expression>,
  location: Location,
  _eval: EvalFn,
) -> Result<Value, InterpreterError> {
  if !args.is_empty() {
    return Err(InterpreterError::ArgumentMismatchError(
      location,
      "pi takes no arguments".to_string(),
    ));
  }
  Ok(Value::Float(std::f64::consts::PI))
}
