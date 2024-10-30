use libs::APP_HANDLE;
use tauri::Listener;
use types::{Data, Expr, InterpreterError, LabeledExpr};

use crate::Interpreter;

impl Interpreter {
  pub fn eval_for_loop(
    &self,
    initializer: Box<Expr>,
    condition: Box<Expr>,
    update: Box<Expr>,
    body: Vec<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    self.eval(LabeledExpr {
      expr: *initializer.clone(),
      line_number: line,
    })?;

    let has_received_event = std::sync::Arc::new(std::sync::Mutex::new(false));
    let has_received_event_clone = std::sync::Arc::clone(&has_received_event);

    // Listen for the break_exec event
    APP_HANDLE
      .lock()
      .unwrap()
      .as_ref()
      .unwrap()
      .listen("break_exec", move |_| {
        let mut should_break = has_received_event_clone.lock().unwrap();
        *should_break = true;
      });

    // 2. Loop while the condition is true
    while match self.eval(LabeledExpr {
      expr: *condition.clone(),
      line_number: line,
    })? {
      Data::Boolean(b) => b,
      _ => {
        return Err(InterpreterError::EvalError(
          line,
          "Condição do loop deve ser booleana.".to_string(),
        ));
      }
    } {
      if *has_received_event.lock().unwrap() {
        break;
      }

      self.eval_block(body.clone())?;

      self.eval(LabeledExpr {
        expr: *update.clone(),
        line_number: line,
      })?;
    }

    Ok(Data::None)
  }
}
