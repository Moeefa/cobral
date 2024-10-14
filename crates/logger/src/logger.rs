#[allow(non_snake_case)]
#[allow(unreachable_code)]
#[allow(unused_imports)]
pub mod Logger {
  use colored::Colorize;
  use libs::APP_HANDLE;
  use serde::{Deserialize, Serialize};
  use tauri::Emitter;
  use types::InterpreterError;

  #[derive(Serialize, Deserialize, Clone)]
  struct Payload {
    pub message: String,
    pub level: String,
  }

  pub fn error(msg: InterpreterError) {
    eprintln!("{}", "🐛 Ocorreu um erro:".on_red());

    eprintln!("{}", msg.to_string().on_red());
    eprintln!("{}", "\t🔍 Detalhes:");

    eprintln!("\t{:?}\n", msg);

    let _ = APP_HANDLE.lock().unwrap().as_ref().unwrap().emit(
      "log",
      Payload {
        message: msg.to_string(),
        level: "error".to_string(),
      },
    );
  }

  pub fn info(msg: &str) {
    eprintln!("{}", "\t🗒️ Info:");

    eprintln!("\t{:?}\n", msg);

    let _ = APP_HANDLE.lock().unwrap().as_ref().unwrap().emit(
      "log",
      Payload {
        message: msg.to_string(),
        level: "info".to_string(),
      },
    );
  }
}
