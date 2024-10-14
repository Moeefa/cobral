use serde::{Deserialize, Serialize};
use tauri::{Emitter, Listener};
use types::{Data, Expr};

use crate::APP_HANDLE;

#[derive(Serialize, Deserialize, Clone)]
struct Payload {
  pub message: String,
  pub level: String,
}

pub fn write(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  let output = args
    .iter()
    .map(|arg| {
      let data = eval(arg.clone()); // Correctly passing `Expr` to `eval`
      match data {
        Some(Data::Float(f)) => f.to_string(),
        Some(Data::Integer(i)) => i.to_string(),
        Some(Data::String(s)) => s,
        Some(Data::Boolean(b)) =>
          if b {
            String::from("verdadeiro")
          } else {
            String::from("falso")
          },
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

  let _ = APP_HANDLE.lock().unwrap().as_ref().unwrap().emit(
    "log",
    Payload {
      message: output.clone(),
      level: "info".to_string(),
    },
  );

  Some(Data::String(output))
}

pub fn read(args: Vec<Expr>, _: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  let mut msg = String::new();
  if let Some(arg) = args.get(0) {
    msg = arg.to_string();
  }

  let _ = APP_HANDLE
    .lock()
    .unwrap()
    .as_ref()
    .unwrap()
    .emit("read", msg.clone());

  // Use Arc<Mutex<String>> to store the input received from the front-end
  let event_received = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
  let event_received_clone = std::sync::Arc::clone(&event_received);

  // Use Arc<Mutex<bool>> to signal when the event is received
  let has_received_event = std::sync::Arc::new(std::sync::Mutex::new(false));
  let has_received_event_clone = std::sync::Arc::clone(&has_received_event);

  // Listen for the "read_input" event from the front-end
  APP_HANDLE
    .lock()
    .unwrap()
    .as_ref()
    .unwrap()
    .listen("read_input", move |msg| {
      let mut received_data = event_received_clone.lock().unwrap();
      *received_data = msg.payload().trim_matches('"').to_string();

      let mut received_flag = has_received_event_clone.lock().unwrap();
      *received_flag = true;
    });

  // Wait for the event to be received
  loop {
    if *has_received_event.lock().unwrap() {
      break;
    }
    std::thread::sleep(std::time::Duration::from_millis(100));
  }

  // Return the received input as Data::String
  let final_input = event_received.lock().unwrap().clone();
  Some(Data::String(final_input))
}
