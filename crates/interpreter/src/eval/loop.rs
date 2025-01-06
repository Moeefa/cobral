use ::enums::{Data, Expr, LabeledExpr};
use libs::APP_HANDLE;
use tauri::Listener;

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_for_loop(
    &self,
    initializer: Box<Expr>,
    condition: Box<Expr>,
    update: Box<Expr>,
    body: Vec<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    // Extract initialization variable name upfront if it's a Let expression
    let cleanup_var = match *initializer {
      Expr::Let(ref name, _) => Some(name.clone()),
      _ => None,
    };

    // Initialize
    self.eval(LabeledExpr {
      expr: *initializer,
      line_number: line,
    })?;

    // Set up break detection
    let should_break = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let should_break_clone = should_break.clone();

    // Listen for break_exec event
    let unlocked_handle = APP_HANDLE.lock().unwrap();
    let app = unlocked_handle.as_ref().unwrap();
    let _listener = app.listen("break_exec", move |_| {
      should_break_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    });

    drop(unlocked_handle); // Release the lock

    let labeled_condition = LabeledExpr {
      expr: *condition,
      line_number: line,
    };

    let labeled_update = LabeledExpr {
      expr: *update,
      line_number: line,
    };

    // Main loop
    while !should_break.load(std::sync::atomic::Ordering::SeqCst) {
      match self.eval(labeled_condition.clone())? {
        Data::Boolean(false) => break,
        Data::Boolean(true) => (),
        _ => {
          return Err(InterpreterError::EvalError(
            line,
            "Condição do loop deve ser booleana".to_string(),
          ))
        }
      }

      self.eval_block(body.clone())?;
      self.eval(labeled_update.clone())?;
    }

    // Cleanup
    if let Some(name) = cleanup_var {
      self.variables.lock().unwrap().remove(&name);
    }

    Ok(Data::None)
  }
}
