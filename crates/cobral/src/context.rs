use interpreter::Interpreter;
use lexer::Lexer;
use libs::AppHandleManager;
use logger::{batch::LogBatchManager, Payload};
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
    let handle = AppHandleManager.get_handle().unwrap();

    let notify_clone = Arc::clone(&has_received_event);
    let handle_clone = handle.clone();
    let id = handle.listen("break_exec", move |_| {
      notify_clone.notify_one();
      handle_clone
        .get_webview_window("main")
        .unwrap()
        .request_user_attention(Some(UserAttentionType::Informational))
        .unwrap();

      LogBatchManager.process_batch();
    });

    // Offload the evaluation task to a background thread
    let input_clone = input.clone();
    task::spawn(async move {
      fn finish_exec(handle: &AppHandle, start: std::time::Instant) {
        let log_batch_manager = LogBatchManager;
        let _ = log_batch_manager.add(Payload {
          message: format!("Tempo de execução: {:?}", start.elapsed()),
          level: String::from("info"),
        });

        handle.emit("exec_finished", ()).unwrap();
        log_batch_manager.process_batch();
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
          finish_exec(&handle, start);

          return;
        }
        _ = async {} => { /* Continue execution */ }
      }

      if let Err(e) = interpreter {
        logger::error(e);
        handle.emit("break_exec", ()).unwrap();
      }

      handle.unlisten(id);
      finish_exec(&handle, start);
    });
  }
}
