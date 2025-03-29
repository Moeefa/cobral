use crate::{
  interpreter::{error::InterpreterError, value::Value},
  shared::ast::Location,
};

pub fn int(args: Vec<Value>, location: Location) -> Result<Value, InterpreterError> {
  let arg = args.get(0).ok_or(InterpreterError::ArgumentMismatchError(
    location.clone(),
    "int requires one argument".to_string(),
  ))?;

  match arg.clone() {
    Value::Float(f) => Ok(Value::Integer(f as i64)),
    Value::Integer(i) => Ok(Value::Integer(i)),
    Value::String(s) => s.parse::<i64>().map(Value::Integer).map_err(|_| {
      InterpreterError::TypeError(location, "Could not parse string as integer".to_string())
    }),
    _ => Err(InterpreterError::TypeError(
      location,
      "Cannot convert to integer".to_string(),
    )),
  }
}

pub fn float(args: Vec<Value>, location: Location) -> Result<Value, InterpreterError> {
  let arg = args.get(0).ok_or(InterpreterError::ArgumentMismatchError(
    location.clone(),
    "float requires one argument".to_string(),
  ))?;

  match arg.clone() {
    Value::Float(f) => Ok(Value::Float(f)),
    Value::Integer(i) => Ok(Value::Float(i as f64)),
    Value::String(s) => s.parse::<f64>().map(Value::Float).map_err(|_| {
      InterpreterError::TypeError(location, "Could not parse string as float".to_string())
    }),
    _ => Err(InterpreterError::TypeError(
      location,
      "Cannot convert to float".to_string(),
    )),
  }
}
