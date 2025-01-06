use std::sync::{Arc, Mutex};

use ::enums::LabeledExpr;
use interpreter::Interpreter;
use lexer::Lexer;
use libs::APP_HANDLE;
use logger::{emit_logs, Payload, LOG_BUFFER};
use parser::Parser;
use tauri::{AppHandle, Emitter, Listener, Runtime};

pub struct Context {
  pub input: Arc<Mutex<String>>,
  pub exprs: Arc<Mutex<Vec<LabeledExpr>>>,
  pub interpreter: Arc<Mutex<Interpreter>>,
  pub start: Arc<Mutex<std::time::Instant>>,
}

impl Context {
  pub fn new() -> Self {
    Context {
      input: Arc::new(Mutex::new(String::new())),
      exprs: Arc::new(Mutex::new(Vec::new())),
      interpreter: Arc::new(Mutex::new(Interpreter::new())),
      start: Arc::new(Mutex::new(std::time::Instant::now())),
    }
  }

  pub fn update(&self, input: String) {
    *self.input.lock().unwrap_or_else(|e| e.into_inner()) = input;
    *self.exprs.lock().unwrap_or_else(|e| e.into_inner()) = Vec::new();
    *self.interpreter.lock().unwrap_or_else(|e| e.into_inner()) = Interpreter::new();
    *self.start.lock().unwrap_or_else(|e| e.into_inner()) = std::time::Instant::now();
  }

  pub fn eval<R: Runtime>(&self, app: AppHandle<R>, input: String) {
    self.reset();
    self.update(input.clone());

    let has_received_event = Arc::new(Mutex::new(false));
    let has_received_event_clone = Arc::clone(&has_received_event);

    // Listen for the "break_exec" event before starting the execution loop
    let app_handle = APP_HANDLE.lock().unwrap().as_ref().unwrap().clone();
    app_handle.listen("break_exec", move |_| {
      let mut should_break = has_received_event_clone.lock().unwrap();
      *should_break = true;
    });

    let start = self.start.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let lexer = Lexer::new(&input.as_str());
    let mut parser = Parser::new(lexer);

    let exprs = self.parse(&mut parser);

    let interpreter = self.interpreter.lock().unwrap_or_else(|e| e.into_inner());

    // Execution loop
    for expr in &exprs {
      // Check the break condition before processing each expression
      if *has_received_event.lock().unwrap() {
        break;
      }

      if let Err(e) = interpreter.eval(expr.clone()) {
        logger::error(e);
        app_handle.emit("break_exec", ()).unwrap();
        break;
      }
    }

    drop(interpreter);
    drop(exprs);

    let mut buffer = LOG_BUFFER.lock().unwrap();
    buffer.push(Payload {
      message: format!("Tempo de execução: {:?}", start.elapsed()),
      level: String::from("info"),
    });
    drop(buffer);

    app_handle.emit("exec_finished", ()).unwrap();
    emit_logs(&app, true);
    drop(app_handle);
  }

  fn parse(&self, parser: &mut Parser) -> Vec<LabeledExpr> {
    let mut exprs = Vec::new();

    loop {
      match parser.parse() {
        Ok(Some(expr)) => exprs.push(LabeledExpr {
          expr: expr,
          line_number: parser.current_token.line_number,
        }),

        Ok(None) => break,

        Err(e) => {
          logger::error(e);
          let app_handle = APP_HANDLE.lock().unwrap().as_ref().unwrap().clone();
          app_handle.emit("break_exec", ()).unwrap();
          drop(app_handle);
          break;
        }
      }
    }

    exprs
  }

  pub fn reset(&self) {
    *self.exprs.lock().unwrap_or_else(|e| e.into_inner()) = Vec::new();
    *self.interpreter.lock().unwrap_or_else(|e| e.into_inner()) = Interpreter::new();
    *self.start.lock().unwrap_or_else(|e| e.into_inner()) = std::time::Instant::now();
  }
}
