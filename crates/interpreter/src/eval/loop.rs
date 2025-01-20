use ::enums::{Data, Expr, LabeledExpr};
use libs::AppHandleManager;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::Listener;

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_for_loop(
    &self,
    initializer: Expr,
    condition: Expr,
    update: Expr,
    body: Vec<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    // Pre-allocate labeled expressions to avoid cloning in the loop
    let labeled_condition = LabeledExpr {
      expr: condition,
      line_number: line,
    };
    let labeled_update = LabeledExpr {
      expr: update,
      line_number: line,
    };

    // Extract initialization variable name upfront
    let cleanup_var = if let Expr::Let(ref name, _) = initializer {
      Some(name.clone())
    } else {
      None
    };

    // Initialize
    self.eval(LabeledExpr {
      expr: initializer,
      line_number: line,
    })?;

    // Use a local AtomicBool for break detection
    let should_break = Arc::new(AtomicBool::new(false));
    let should_break_clone = should_break.clone();

    let handle = AppHandleManager.get_handle().unwrap();
    let id = handle.listen("break_exec", move |_| {
      should_break_clone.store(true, Ordering::Release);
    });

    // Main loop with optimized condition checking
    while !should_break.load(Ordering::Acquire) {
      match self.eval(labeled_condition.clone())? {
        Data::Boolean(false) => break,
        Data::Boolean(true) => {
          self.eval_block(&body)?; // Pass reference instead of cloning
          self.eval(labeled_update.clone())?;
        }
        _ => {
          handle.unlisten(id);
          return Err(InterpreterError::EvalError(
            line,
            "Condição do loop deve ser booleana".into(),
          ));
        }
      }
    }

    // Cleanup
    handle.unlisten(id);
    if let Some(name) = cleanup_var {
      self.env.variables.write().remove(&name);
    }

    Ok(Data::None)
  }
}
