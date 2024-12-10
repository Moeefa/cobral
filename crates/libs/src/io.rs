use std::sync::{Arc, Mutex};

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
  emit_logs(APP_HANDLE.lock().unwrap().as_ref().unwrap(), true);

  // Prepare the message to be sent
  let msg = args.get(0).map_or(String::new(), |arg| arg.to_string());

  // Emit the message to the front end
  let _ = APP_HANDLE
    .lock()
    .unwrap()
    .as_ref()
    .unwrap()
    .emit("read", msg.clone());

  // Use Arc<Mutex<Option<String>>> for input received from the front-end
  let received_input = Arc::new(Mutex::new(None));
  let received_input_clone = Arc::clone(&received_input);

  // Use Arc<Mutex<bool>> for break signal
  let break_signal = Arc::new(Mutex::new(false));
  let break_signal_clone = Arc::clone(&break_signal);

  // Listener for the "read_input" event
  let app_handle = APP_HANDLE.lock().unwrap().as_ref().unwrap().clone();
  app_handle.listen("read_input", move |msg| {
    let mut input = received_input_clone.lock().unwrap();
    *input = Some(msg.payload().trim_matches('"').to_string());
  });

  // Listener for the "break_read" event
  let app_handle_break = APP_HANDLE.lock().unwrap().as_ref().unwrap().clone();
  app_handle_break.listen("break_read", move |_| {
    let mut signal = break_signal_clone.lock().unwrap();
    *signal = true;
  });

  // Wait for either input or break signal
  loop {
    if *break_signal.lock().unwrap() {
      return None; // Break the loop and terminate
    }

    if let Some(input) = received_input.lock().unwrap().clone() {
      return Some(Data::String(input)); // Return valid input
    }

    std::thread::sleep(std::time::Duration::from_millis(100)); // Avoid busy waiting
  }
}
