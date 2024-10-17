use std::sync::{Arc, LazyLock, Mutex};

use interpreter::Interpreter;
use lexer::Lexer;
use logger::Logger;
use parser::Parser;
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Emitter, Runtime, Window};
use types::{InterpreterError, LabeledExpr};

#[derive(Serialize, Deserialize, Clone)]
struct Payload {
  pub message: String,
  pub level: String,
}

static INPUT: LazyLock<Arc<Mutex<String>>> = LazyLock::new(|| Arc::new(Mutex::new(String::new())));
static EXPRS: LazyLock<Arc<Mutex<Vec<LabeledExpr>>>> =
  LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));
static INTERPRETER: LazyLock<Arc<Mutex<Interpreter>>> =
  LazyLock::new(|| Arc::new(Mutex::new(Interpreter::new())));
static START: LazyLock<Arc<Mutex<std::time::Instant>>> =
  LazyLock::new(|| Arc::new(Mutex::new(std::time::Instant::now())));

#[command]
pub async fn update<R: Runtime>(_app: AppHandle<R>, _window: Window<R>, input: String) {
  *INPUT.lock().unwrap_or_else(|e| e.into_inner()) = input;
  *EXPRS.lock().unwrap_or_else(|e| e.into_inner()) = Vec::new();
  *INTERPRETER.lock().unwrap_or_else(|e| e.into_inner()) = Interpreter::new();
  *START.lock().unwrap_or_else(|e| e.into_inner()) = std::time::Instant::now();
}

#[command]
pub async fn step<R: Runtime>(app: AppHandle<R>, _window: Window<R>) -> usize {
  let input = INPUT.lock().unwrap_or_else(|e| e.into_inner()).clone();
  let lexer = Lexer::new(&input.as_str());
  let mut parser = Parser::new(lexer);

  if EXPRS
    .lock()
    .unwrap_or_else(|e| e.into_inner())
    .clone()
    .is_empty()
  {
    app.emit("clear", ()).unwrap();
    *INTERPRETER.lock().unwrap_or_else(|e| e.into_inner()) = Interpreter::new();
    *EXPRS.lock().unwrap_or_else(|e| e.into_inner()) = parse_all(&mut parser);
    *START.lock().unwrap_or_else(|e| e.into_inner()) = std::time::Instant::now();
  }

  // Create an Interpreter instance
  let interpreter = INTERPRETER.lock().unwrap();

  match interpreter.eval(
    EXPRS
      .lock()
      .unwrap_or_else(|e| e.into_inner())
      .clone()
      .first()
      .unwrap()
      .clone(),
  ) {
    Ok(_) => {}
    Err(e) => {
      Logger::error(InterpreterError::ParseError(
        parser.current_token.line_number,
        e.to_string(),
      ));
    }
  }

  if EXPRS.lock().unwrap().len() == 1 {
    let elapsed = START
      .lock()
      .unwrap_or_else(|e| e.into_inner())
      .clone()
      .elapsed();
    app
      .emit(
        "log",
        Payload {
          message: format!("Tempo de execução: {:?}", elapsed),
          level: String::from("info"),
        },
      )
      .unwrap();
  }

  EXPRS
    .lock()
    .unwrap_or_else(|e| e.into_inner())
    .clone()
    .remove(0);

  EXPRS
    .lock()
    .unwrap_or_else(|e| e.into_inner())
    .clone()
    .len()
}

#[command]
pub async fn parse<R: Runtime>(app: AppHandle<R>, _window: Window<R>, input: String) {
  *EXPRS.lock().unwrap_or_else(|e| e.into_inner()) = Vec::new();
  *INTERPRETER.lock().unwrap_or_else(|e| e.into_inner()) = Interpreter::new();
  *START.lock().unwrap_or_else(|e| e.into_inner()) = std::time::Instant::now();

  let start = std::time::Instant::now();
  let lexer = Lexer::new(&input.as_str());
  let mut parser = Parser::new(lexer);

  let exprs = parse_all(&mut parser);

  // Create an Interpreter instance
  let interpreter = Interpreter::new();

  // Evaluate each expression and print the result
  for expr in exprs {
    match interpreter.eval(expr) {
      Ok(_) => {}
      Err(e) => {
        Logger::error(InterpreterError::ParseError(
          parser.current_token.line_number,
          e.to_string(),
        ));

        break;
      }
    }
  }

  let elapsed = start.elapsed();
  app
    .emit(
      "log",
      Payload {
        message: format!("Tempo de execução: {:?}", elapsed),
        level: String::from("info"),
      },
    )
    .unwrap();
}

fn parse_all(parser: &mut Parser) -> Vec<LabeledExpr> {
  let mut exprs = Vec::new();

  loop {
    match parser.parse() {
      Ok(Some(expr)) => exprs.push(LabeledExpr {
        expr: expr,
        line_number: parser.current_token.line_number,
      }),
      Ok(None) => break,
      Err(e) => {
        Logger::error(InterpreterError::ParseError(
          parser.current_token.line_number,
          e.to_string(),
        ));

        break;
      }
    }
  }

  exprs
}
