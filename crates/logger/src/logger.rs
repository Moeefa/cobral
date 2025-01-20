pub mod batch;

use batch::LogBatchManager;
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Payload {
  pub message: String,
  pub level: String,
}

pub fn error(msg: impl ToString) {
  eprintln!("{}", "🐛 Ocorreu um erro:".on_red());

  eprintln!("{}", msg.to_string().on_red());
  eprintln!("{}", "\t🔍 Detalhes:");

  eprintln!("\t{}\n", msg.to_string());

  let _ = LogBatchManager.add(Payload {
    message: msg.to_string(),
    level: String::from("error"),
  });
}

pub fn info(msg: impl ToString) {
  eprintln!("{}", "\t🗒️ Info:");

  eprintln!("\t{}\n", msg.to_string());

  let msg_clone = msg.to_string();
  let _ = LogBatchManager.add(Payload {
    message: msg_clone,
    level: String::from("info"),
  });
}
