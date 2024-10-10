use colored::Colorize;
use libs::APP_HANDLE;
use tauri::Emitter;
use types::InterpreterError;

pub fn error(msg: InterpreterError) {
  eprintln!("{}", "🐛 Ocorreu um erro:".on_red());

  eprintln!("{}", msg.to_string().on_red());
  eprintln!("{}", "\t🔍 Detalhes:");

  eprintln!("\t{:?}\n", msg);

  let _ = APP_HANDLE
    .lock()
    .unwrap()
    .as_ref()
    .unwrap()
    .emit("log", msg.to_string());
}

pub fn info(msg: &str) {
  eprintln!("{}", "\t🗒️ Info:");

  eprintln!("\t{:?}\n", msg);

  let _ = APP_HANDLE
    .lock()
    .unwrap()
    .as_ref()
    .unwrap()
    .emit("log", msg.to_string());
}

#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {
    print!($($arg)*);
  };
}

#[macro_export]
macro_rules! log_warn {
  ($($arg:tt)*) => {
    print!($($arg)*);
  };
}
