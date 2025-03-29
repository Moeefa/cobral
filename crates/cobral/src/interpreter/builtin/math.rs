use crate::{
  interpreter::{error::InterpreterError, value::Value},
  shared::ast::Location,
};

pub fn square_root(args: Vec<Value>, location: Location) -> Result<Value, InterpreterError> {
  if args.len() != 1 {
    return Err(InterpreterError::ArgumentMismatchError(
      location,
      "square_root requires one argument".to_string(),
    ));
  }

  let arg = args.get(0).ok_or(InterpreterError::ArgumentMismatchError(
    location.clone(),
    "square_root requires one argument".to_string(),
  ))?;

  match arg.clone() {
    Value::Float(f) => Ok(Value::Float(f.sqrt())),
    Value::Integer(i) => Ok(Value::Float((i as f64).sqrt())),
    _ => Err(InterpreterError::TypeError(
      location,
      "Expected number for square_root".to_string(),
    )),
  }
}

pub fn power(args: Vec<Value>, location: Location) -> Result<Value, InterpreterError> {
  if args.len() != 2 {
    return Err(InterpreterError::ArgumentMismatchError(
      location,
      "Potência requer dois argumentos".to_string(),
    ));
  }

  let base = args.get(0).ok_or(InterpreterError::ArgumentMismatchError(
    location.clone(),
    "Potência requer dois argumentos".to_string(),
  ))?;
  let exponent = args.get(1).ok_or(InterpreterError::ArgumentMismatchError(
    location.clone(),
    "Potência requer dois argumentos".to_string(),
  ))?;

  println!("base: {:?}, exponent: {:?}", base, exponent);

  match (base, exponent) {
    (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1.powf(*f2))),
    (Value::Float(f), Value::Integer(i)) => Ok(Value::Float(f.powi(*i as i32))),
    (Value::Integer(i), Value::Float(f)) => Ok(Value::Float((*i as f64).powf(*f))),
    (Value::Integer(i1), Value::Integer(i2)) => Ok(Value::Integer(i1.pow(*i2 as u32))),
    (b, e) => Err(InterpreterError::TypeError(
      location,
      format!(
        "Era esperado um número inteiro ou real, recebeu-se '{0}' e '{1}'.",
        b, e
      )
      .to_string(),
    )),
  }
}

pub fn pi(args: Vec<Value>, location: Location) -> Result<Value, InterpreterError> {
  if !args.is_empty() {
    return Err(InterpreterError::ArgumentMismatchError(
      location,
      "pi takes no arguments".to_string(),
    ));
  }

  Ok(Value::Float(std::f64::consts::PI))
}
