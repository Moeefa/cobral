use std::sync::Arc;
use tauri::{AppHandle, Emitter, Listener, Manager, UserAttentionType};
use tokio::{sync::Notify, task};

use crate::{
  interpreter::Interpreter,
  lexer::Lexer,
  logger::{self, batcher::LogBatchManager},
  parser::Parser,
  shared::AppHandleManager,
};

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
      let start = std::time::Instant::now();

      let finish_exec = |handle: &AppHandle| {
        handle.unlisten(id);
        logger::info(format!("Tempo de execução: {:?}", start.elapsed()));

        handle.emit("exec_finished", ()).unwrap();
        LogBatchManager.process_batch();
      };

      let break_exec = |handle: &AppHandle, e: String| {
        logger::error(e);

        finish_exec(handle);

        handle.emit("break_exec", ()).unwrap();
      };

      let tokens = match Lexer::new(&input_clone) {
        Ok(tokens) => tokens,
        Err(e) => {
          break_exec(&handle, e.to_string());
          return;
        }
      };

      let exprs = match Parser::new(tokens) {
        Ok(exprs) => exprs,
        Err(e) => {
          break_exec(&handle, e.to_string());
          return;
        }
      };

      let _interpreter = match Interpreter::new(exprs) {
        Ok(interpreter) => interpreter,
        Err(e) => {
          break_exec(&handle, e.to_string());
          return;
        }
      };

      // Wait for the `break_exec` notification or continue normally
      tokio::select! {
        _ = has_received_event.notified() => {
          finish_exec(&handle);

          return;
        }
        _ = async {} => { /* Continue execution */ }
      }

      finish_exec(&handle);
    });
  }
}
