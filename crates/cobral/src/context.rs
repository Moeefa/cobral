use interpreter::Interpreter;
use lexer::Lexer;
use libs::APP_HANDLE;
use logger::{emit_logs, Payload, LOG_BUFFER};
use parser::Parser;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Listener, Manager, UserAttentionType};
use tokio::{sync::Notify, task};

pub struct ExecutionContext {}

impl ExecutionContext {
  pub fn new() -> Self {
    ExecutionContext {}
  }

  pub async fn eval(&self, input: String) {
    let has_received_event = Arc::new(Notify::new());
    let app_handle = APP_HANDLE.lock().unwrap().as_ref().unwrap().clone();

    // Listen for the "break_exec" event
    {
      let notify_clone = Arc::clone(&has_received_event);
      let app_handle_clone = app_handle.clone();
      app_handle.listen("break_exec", move |_| {
        notify_clone.notify_one();
        app_handle_clone
          .get_webview_window("main")
          .unwrap()
          .request_user_attention(Some(UserAttentionType::Informational))
          .unwrap();
        emit_logs(&app_handle_clone, true);
      });
    }

    // Offload the evaluation task to a background thread
    let input_clone = input.clone();
    task::spawn(async move {
      fn finish_exec(app_handle: AppHandle, start: std::time::Instant) {
        let mut buffer = LOG_BUFFER.lock().unwrap();
        buffer.push(Payload {
          message: format!("Tempo de execução: {:?}", start.elapsed()),
          level: String::from("info"),
        });
        drop(buffer);

        app_handle.emit("exec_finished", ()).unwrap();
        emit_logs(&app_handle, true);
      }

      let start = std::time::Instant::now();
      let lexer = Lexer::new(&input_clone);
      let parser = Parser::new(lexer);
      let exprs = parser.unwrap_or_else(|e| {
        logger::error(e);
        Vec::new()
      });
      let interpreter = Interpreter::new(exprs.clone());

      drop(exprs);

      // Wait for the `break_exec` notification or continue normally
      tokio::select! {
        _ = has_received_event.notified() => {
          finish_exec(app_handle, start);
          return;
        }
        _ = async {} => { /* Continue execution */ }
      }

      if let Err(e) = interpreter {
        logger::error(e);
        app_handle.emit("break_exec", ()).unwrap();
      }

      finish_exec(app_handle, start);
    });
  }
}
