use std::sync::{Arc, Condvar, Mutex};

use logger::batcher::LogBatchManager;
use tauri::{Emitter, Listener};

use crate::{
  interpreter::{error::InterpreterError, value::Value, EvalFn},
  logger,
  shared::ast::{Expression, Location},
  AppHandleManager,
};

fn args_to_string(args: Vec<Expression>, eval: EvalFn) -> Result<Value, InterpreterError> {
  let mut output: Vec<String> = Vec::new();

  for arg in args {
    match eval(arg) {
      Ok(value) => output.push(value.to_string()),
      Err(e) => return Err(e),
    }
  }

  Ok(Value::String(output.join(" ")))
}

pub fn write(
  args: Vec<Expression>,
  _location: Location,
  eval: EvalFn,
) -> Result<Value, InterpreterError> {
  let output = args_to_string(args, eval)?;
  logger::info(&output);

  Ok(output)
}

pub fn error(
  args: Vec<Expression>,
  _location: Location,
  eval: EvalFn,
) -> Result<Value, InterpreterError> {
  let output = args_to_string(args, eval)?;
  logger::error(&output);

  Ok(output)
}

pub fn read(
  args: Vec<Expression>,
  location: Location,
  eval: EvalFn,
) -> Result<Value, InterpreterError> {
  let handle = AppHandleManager.get_handle().or_else(|_| {
    Err(InterpreterError::RuntimeError(
      location,
      "Failed to get app handle".to_string(),
    ))
  })?;

  let output = args_to_string(args, eval)?;

  LogBatchManager.process_batch();

  let _ = handle.emit("read", output);

  let shared_state: Arc<(Mutex<Option<Result<Value, InterpreterError>>>, Condvar)> =
    Arc::new((Mutex::new(None), Condvar::new()));
  let shared_state_clone = Arc::clone(&shared_state);

  let read_listener = handle.listen("read_input", move |msg| {
    let (lock, cvar) = &*shared_state_clone;
    let mut input = lock.lock().unwrap();
    *input = Some(Ok(Value::String(
      msg.payload().trim_matches('"').to_string(),
    )));
    cvar.notify_one();
  });

  let shared_state_clone = Arc::clone(&shared_state);
  let break_listener = handle.listen("break_exec", move |_| {
    let (lock, cvar) = &*shared_state_clone;
    let mut input = lock.lock().unwrap();
    *input = Some(Ok(Value::None));
    cvar.notify_one();
  });

  let (lock, cvar) = &*shared_state;
  let mut input = lock.lock().unwrap();
  while input.is_none() {
    input = cvar.wait(input).unwrap();
  }

  handle.unlisten(read_listener);
  handle.unlisten(break_listener);

  input.take().unwrap_or(Err(InterpreterError::RuntimeError(
    location,
    "Failed to get input".to_string(),
  )))
}
