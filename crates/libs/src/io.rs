use std::sync::{Arc, Condvar, Mutex};

use logger::{emit_logs, Payload, LOG_BUFFER};
use tauri::{Emitter, Listener};
use types::{Data, Expr};

use crate::APP_HANDLE;

pub fn write(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  let output = args
    .iter()
    .map(|arg| {
      let data = eval(arg.clone()); // Correctly passing `Expr` to `eval`
      match data {
        Some(Data::Float(f)) => f.to_string(),
        Some(Data::Integer(i)) => i.to_string(),
        Some(Data::String(s)) => s,
        Some(Data::Boolean(b)) => {
          if b {
            String::from("verdadeiro")
          } else {
            String::from("falso")
          }
        }
        Some(Data::Undefined) => String::from("Indefinido"),
        Some(Data::List(l)) => {
          let mut output = String::from("");
          for (i, item) in l.iter().enumerate() {
            output.push_str(&match item {
              Data::Float(f) => f.to_string(),
              Data::Integer(i) => i.to_string(),
              Data::String(s) => s.to_string(),
              Data::Boolean(b) => b.to_string(),
              Data::Undefined => String::from("Indefinido"),
              Data::List(_) => String::from("Lista"),

              #[allow(unreachable_patterns)]
              _ => String::from("Tipo desconhecido"),
            });
            if i < l.len() - 1 {
              output.push_str(", ");
            }
          }
          output
        }

        #[allow(unreachable_patterns)]
        _ => String::from("Tipo desconhecido"),
      }
    })
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

pub fn read(args: Vec<Expr>, _: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  let app_handle = APP_HANDLE.lock().unwrap().as_ref().unwrap().clone();

  // Prepare the message to send to the front end
  let msg = args.get(0).map_or(String::new(), |arg| {
    let arg_str = arg.to_string();
    arg_str 
      .strip_prefix('"')
      .and_then(|s| s.strip_suffix('"'))
      .unwrap_or(&arg_str)
      .to_string()
  });
  let _ = app_handle.emit("read", msg.clone());

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
    input = cvar.wait(input).unwrap(); // Wait for a notification
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
