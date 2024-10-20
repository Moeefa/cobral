use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc, Mutex,
};

use interpreter::Interpreter;
use lexer::Lexer;
use libs::APP_HANDLE;
use logger::{emit_logs, Payload, LOG_BUFFER};
use parser::Parser;
use tauri::{AppHandle, Emitter, Listener, Runtime};
use types::{InterpreterError, LabeledExpr};

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

  pub fn step<R: Runtime>(&self, app: AppHandle<R>) -> usize {
    let input = self.input.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let lexer = Lexer::new(&input.as_str());
    let mut parser = Parser::new(lexer);

    if self
      .exprs
      .lock()
      .unwrap_or_else(|e| e.into_inner())
      .is_empty()
    {
      app.emit("clear", ()).unwrap();
      self.reset();

      *self.exprs.lock().unwrap_or_else(|e| e.into_inner()) = self.parse_all(&mut parser);
    }

    let interpreter = self.interpreter.lock().unwrap_or_else(|e| e.into_inner());

    match interpreter.eval(
      self
        .exprs
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone()
        .first()
        .unwrap()
        .clone(),
    ) {
      Ok(_) => {}
      Err(e) => {
        logger::error(InterpreterError::ParseError(
          parser.current_token.line_number,
          e.to_string(),
        ));
      }
    }

    if self.exprs.lock().unwrap_or_else(|e| e.into_inner()).len() == 1 {
      let elapsed = self
        .start
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .elapsed();

      LOG_BUFFER
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .push(Payload {
          message: format!("Tempo de execução: {:?}", elapsed),
          level: String::from("info"),
        });

      emit_logs(&app, true);
    }

    self
      .exprs
      .lock()
      .unwrap_or_else(|e| e.into_inner())
      .remove(0);

    self.exprs.lock().unwrap_or_else(|e| e.into_inner()).len()
  }

  pub fn parse<R: Runtime>(&self, app: AppHandle<R>, input: String) {
    self.reset();
    self.update(input.clone());

    let start = self.start.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let lexer = Lexer::new(&input.as_str());
    let mut parser = Parser::new(lexer);

    *self.exprs.lock().unwrap_or_else(|e| e.into_inner()) = self.parse_all(&mut parser);

    let interpreter = self.interpreter.lock().unwrap_or_else(|e| e.into_inner());

    let has_received_event = Arc::new(Mutex::new(false));
    let has_received_event_clone = Arc::clone(&has_received_event);

    // Listen for the "break_exec" event before starting the execution loop
    APP_HANDLE
      .lock()
      .unwrap()
      .as_ref()
      .unwrap()
      .listen("break_exec", move |_| {
        let mut should_break = has_received_event_clone.lock().unwrap();
        *should_break = true;
      });

    for expr in self.exprs.lock().unwrap_or_else(|e| e.into_inner()).clone() {
      if *has_received_event.lock().unwrap() {
        self.reset();
        break;
      }

      match interpreter.eval(expr) {
        Ok(_) => {}
        Err(e) => {
          logger::error(InterpreterError::ParseError(
            parser.current_token.line_number,
            e.to_string(),
          ));
          break;
        }
      }
    }

    let elapsed = start.elapsed();
    LOG_BUFFER
      .lock()
      .unwrap_or_else(|e| e.into_inner())
      .push(Payload {
        message: format!("Tempo de execução: {:?}", elapsed),
        level: String::from("info"),
      });

    emit_logs(&app, true);
  }

  fn parse_all(&self, parser: &mut Parser) -> Vec<LabeledExpr> {
    let mut exprs = Vec::new();

    loop {
      match parser.parse() {
        Ok(Some(expr)) => exprs.push(LabeledExpr {
          expr: expr,
          line_number: parser.current_token.line_number,
        }),
        Ok(None) => break,
        Err(e) => {
          logger::error(InterpreterError::ParseError(
            parser.current_token.line_number,
            e.to_string(),
          ));
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
