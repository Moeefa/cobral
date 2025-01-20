use std::sync::{Arc, Condvar, Mutex};

use ::enums::{Data, Expr};
use logger::{batch::LogBatchManager, Payload};
use tauri::{Emitter, Listener};

use crate::AppHandleManager;

fn args_to_string(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<String> {
  let output: Vec<Option<String>> = args
    .iter()
    .map(|arg| {
      let data = eval(arg.clone()); // Correctly passing `Expr` to `eval`
      match data {
        Some(d) => Some(d.to_string()),
        _ => None,
      }
    })
    .collect();

  if output.iter().any(|o| o.is_none()) {
    return None;
  }

  Some(
    output
      .into_iter()
      .map(|o| o.unwrap())
      .collect::<Vec<String>>()
      .join(" "),
  )
}

pub fn write(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  let output = args_to_string(args, eval)?;

  let _ = LogBatchManager.add(Payload {
    message: output.clone(),
    level: String::from("info"),
  });

  Some(Data::String(output))
}

pub fn error(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  let output = args_to_string(args, eval)?;

  let _ = LogBatchManager.add(Payload {
    message: output.clone(),
    level: String::from("info"),
  });

  Some(Data::String(output))
}

pub fn read(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  let handle = AppHandleManager.get_handle().unwrap();

  let output = args_to_string(args, eval)?;

  let _ = handle.emit("read", output);

  // Shared state for input and break signal
  let shared_state: Arc<(Mutex<Option<Data>>, Condvar)> =
    Arc::new((Mutex::new(None), Condvar::new()));
  let shared_state_clone: Arc<(Mutex<Option<Data>>, Condvar)> = Arc::clone(&shared_state);

  // Listener for "read_input" event
  let read_listener = handle.listen("read_input", move |msg| {
    let (lock, cvar) = &*shared_state_clone;
    let mut input = lock.lock().unwrap();
    *input = Some(Data::String(msg.payload().trim_matches('"').to_string()));
    cvar.notify_one(); // Notify the waiting thread
  });

  // Break signal listener
  let shared_state_clone = Arc::clone(&shared_state);
  let break_listener = handle.listen("break_exec", move |_| {
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
  handle.unlisten(read_listener);
  handle.unlisten(break_listener);

  // Handle the received input
  match input.take() {
    Some(Data::String(value)) => Some(Data::String(value)), // Return the received input
    Some(Data::None) => Some(Data::None),                   // Return None on break signal
    _ => None, // Shouldn't happen but included for safety
  }
}
