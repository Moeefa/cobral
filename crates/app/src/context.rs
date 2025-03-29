use crate::utils::AppHandleManager;
use cobral::{
  interpreter::{Interpreter, InterpreterState},
  lexer::Lexer,
  logger::{self, batcher::LogBatchManager},
  parser::Parser,
};
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

      // Create the interpreter and KEEP IT ALIVE
      let mut interpreter = match Interpreter::new(exprs) {
        Ok(interpreter) => interpreter,
        Err(e) => {
          break_exec(&handle, e.to_string());
          return;
        }
      };

      // Set up a channel to receive input data
      let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(10);

      // Clone the sender for the listener
      let tx_clone = tx.clone();

      // Set up a listener for read_input events
      let input_listener_id = handle.listen("read_input", move |event| {
        // Send the input through the channel
        let _ = tx_clone.try_send(event.payload().to_string());
      });

      // Main execution loop
      loop {
        // If the interpreter is waiting for input, we need to wait for the event
        if interpreter.is_waiting_for_input() {
          tokio::select! {
              // Wait for the break execution event
              _ = has_received_event.notified() => {
                  handle.unlisten(input_listener_id);
                  finish_exec(&handle);
                  return;
              }
              // Wait for input data
              Some(input_data) = rx.recv() => {
                  // Process the input
                  match interpreter.provide_input(input_data) {
                      Ok(_) => {
                          // Continue execution
                      },
                      Err(e) => {
                          handle.unlisten(input_listener_id);
                          break_exec(&handle, e.to_string());
                          return;
                      }
                  }
              }
          }
        } else {
          // Check if execution is complete
          match interpreter.get_state() {
            InterpreterState::Completed => {
              handle.unlisten(input_listener_id);
              finish_exec(&handle);
              return;
            }
            InterpreterState::Error(e) => {
              handle.unlisten(input_listener_id);
              break_exec(&handle, e.to_string());
              return;
            }
            _ => {
              // If not waiting or finished, run more statements
              match interpreter.run() {
                Ok(_) => {
                  // Will loop back and check state again
                }
                Err(e) => {
                  handle.unlisten(input_listener_id);
                  break_exec(&handle, e.to_string());
                  return;
                }
              }
            }
          }
        }

        // Process any pending logs
        LogBatchManager.process_batch();
      }
    });
  }
}
