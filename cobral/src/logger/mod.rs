pub mod batcher;

use batcher::LogBatchManager;
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Payload {
  pub message: String,
  pub level: String,
}

pub fn error(msg: impl ToString) {
  eprintln!("{}", "🐛\tOcorreu um erro:");
  eprintln!("{}", msg.to_string().on_red());

  let _ = LogBatchManager.add(Payload {
    message: msg.to_string(),
    level: String::from("error"),
  });
}

pub fn info(msg: impl ToString) {
  eprint!("{}", "🗒️\tInfo: ");
  eprintln!("{}", msg.to_string());

  let msg_clone = msg.to_string();
  let _ = LogBatchManager.add(Payload {
    message: msg_clone,
    level: String::from("info"),
  });
}
