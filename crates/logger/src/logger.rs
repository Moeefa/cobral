use std::sync::{Arc, LazyLock, Mutex};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Runtime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Payload {
  pub message: String,
  pub level: String,
}

pub static LOG_BUFFER: LazyLock<Arc<Mutex<Vec<Payload>>>> =
  LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

pub fn emit_logs<R: Runtime>(app: &AppHandle<R>, force_emit: bool) {
  let mut buffer = LOG_BUFFER.lock().unwrap();
  if force_emit || buffer.len() >= 200 {
    if !buffer.is_empty() {
      app.emit("log_batch", buffer.clone()).unwrap();
      buffer.clear();
    }
  }

  drop(buffer);
}

pub fn error(msg: impl ToString) {
  eprintln!("{}", "🐛 Ocorreu um erro:".on_red());

  eprintln!("{}", msg.to_string().on_red());
  eprintln!("{}", "\t🔍 Detalhes:");

  eprintln!("\t{}\n", msg.to_string());

  let mut buffer = LOG_BUFFER.lock().unwrap();
  buffer.push(Payload {
    message: msg.to_string(),
    level: String::from("error"),
  });

  drop(buffer);
}

pub fn info(msg: impl ToString) {
  eprintln!("{}", "\t🗒️ Info:");

  eprintln!("\t{}\n", msg.to_string());

  let msg_clone = msg.to_string();
  let mut buffer = LOG_BUFFER.lock().unwrap();
  buffer.push(Payload {
    message: msg_clone,
    level: String::from("info"),
  });

  drop(buffer);
}
