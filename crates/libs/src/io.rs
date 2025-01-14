use std::sync::{Arc, Condvar, Mutex};

use ::enums::{Data, Expr};
use logger::{emit_logs, Payload, LOG_BUFFER};
use tauri::{Emitter, Listener};

use crate::APP_HANDLE;

pub fn write(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  let output: Vec<Option<String>> = args
    .iter()
    .map(|arg| {
      let data = eval(arg.clone()); // Correctly passing `Expr` to `eval`
      if let Some(data) = data {
        Some(data.to_string())
      } else {
        None
      }
    })
    .collect();

  if output.iter().any(|o| o.is_none()) {
    return Some(Data::None);
  }

  let output: String = output
    .into_iter()
    .map(|o| o.unwrap())
    .collect::<Vec<String>>()
    .join(" ");

  let mut buffer = LOG_BUFFER.lock().unwrap();
  buffer.push(Payload {
    message: output.clone(),
    level: String::from("info"),
  });
  drop(buffer);

  emit_logs(APP_HANDLE.lock().unwrap().as_ref().unwrap(), false);

  Some(Data::String(output))
}

pub fn error(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  let output: Vec<Option<String>> = args
    .iter()
    .map(|arg| {
      let data = eval(arg.clone()); // Correctly passing `Expr` to `eval`
      if let Some(data) = data {
        Some(data.to_string())
      } else {
        None
      }
    })
    .collect();

  if output.iter().any(|o| o.is_none()) {
    return Some(Data::None);
  }

  let output: String = output
    .into_iter()
    .map(|o| o.unwrap())
    .collect::<Vec<String>>()
    .join(" ");

  let mut buffer = LOG_BUFFER.lock().unwrap();
  buffer.push(Payload {
    message: output.clone(),
    level: String::from("error"),
  });
  drop(buffer);

  emit_logs(APP_HANDLE.lock().unwrap().as_ref().unwrap(), false);

  Some(Data::String(output))
}

pub fn read(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  let app_handle = APP_HANDLE.lock().unwrap().as_ref().unwrap().clone();

  let output: Vec<Option<String>> = args
    .iter()
    .map(|arg| {
      let data = eval(arg.clone()); // Correctly passing `Expr` to `eval`
      if let Some(data) = data {
        Some(data.to_string())
      } else {
        None
      }
    })
    .collect();

  let output: String = output
    .into_iter()
    .map(|o| o.unwrap())
    .collect::<Vec<String>>()
    .join(" ");

  let _ = app_handle.emit("read", output);

  // Shared state for input and break signal
  let shared_state: Arc<(Mutex<Option<Data>>, Condvar)> =
    Arc::new((Mutex::new(None), Condvar::new()));
  let shared_state_clone: Arc<(Mutex<Option<Data>>, Condvar)> = Arc::clone(&shared_state);

  // Listener for "read_input" event
  let read_listener = app_handle.listen("read_input", move |msg| {
    let (lock, cvar) = &*shared_state_clone;
    let mut input = lock.lock().unwrap();
    *input = Some(Data::String(msg.payload().trim_matches('"').to_string()));
    cvar.notify_one(); // Notify the waiting thread
  });

  // Break signal listener
  let shared_state_clone = Arc::clone(&shared_state);
  let break_listener = app_handle.listen("break_exec", move |_| {
    let (lock, cvar) = &*shared_state_clone;
    let mut input = lock.lock().unwrap();
    *input = Some(Data::None);
    cvar.notify_one(); // Notify the waiting thread
  });

  // Wait for input or break signal
  let (lock, cvar) = &*shared_state;
  let mut input = lock.lock().unwrap();
  while input.is_none() {
    input = cvar.wait(input).unwrap();
  }

  // Cleanup listeners
  app_handle.unlisten(read_listener);
  app_handle.unlisten(break_listener);

  // Handle the received input
  match input.take() {
    Some(Data::String(value)) => Some(Data::String(value)), // Return the received input
    Some(Data::None) => Some(Data::None),                   // Return None on break signal
    _ => None, // Shouldn't happen but included for safety
  }
}
