use logger::batcher::LogBatchManager;

use crate::{
  event::GLOBAL_EVENT_SYSTEM,
  interpreter::{error::InterpreterError, value::Value},
  logger,
  shared::ast::Location,
};

pub fn write(args: Vec<Value>, _location: Location) -> Result<Value, InterpreterError> {
  let output = &args
    .iter()
    .map(|v| v.to_string())
    .collect::<Vec<_>>()
    .join(" ");

  logger::info(output);

  Ok(Value::String(output.to_string()))
}

pub fn error(args: Vec<Value>, _location: Location) -> Result<Value, InterpreterError> {
  let output = &args
    .iter()
    .map(|v| v.to_string())
    .collect::<Vec<_>>()
    .join(" ");

  logger::error(output);

  Ok(Value::String(output.to_string()))
}

pub fn read(args: Vec<Value>, _location: Location) -> Result<Value, InterpreterError> {
  let output = &args
    .iter()
    .map(|v| v.to_string())
    .collect::<Vec<_>>()
    .join(" ");

  LogBatchManager.process_batch();

  GLOBAL_EVENT_SYSTEM.emit("spawn_input", output.to_string());

  let callback_id = GLOBAL_EVENT_SYSTEM.register_callback(Box::new(|input| {
    Ok(Value::String(input.trim_matches('"').to_string()))
  }));

  // Return InputPending with the callback ID
  Ok(Value::InputPending(callback_id))
}
