use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::Listener;

use crate::interpreter::error::InterpreterError;
use crate::interpreter::value::Value;
use crate::interpreter::Interpreter;
use crate::shared::ast::{Expression, Statement};
use crate::shared::AppHandleManager;

impl Interpreter {
  pub fn eval_for_stmt(
    &mut self,
    initializer: Statement,
    condition: Expression,
    update: Statement,
    body: Vec<Statement>,
  ) -> Result<Value, InterpreterError> {
    let state = self.environment.symbols.read().clone();

    // Initialize
    self.eval_stmt(initializer)?;

    // Use a local AtomicBool for break detection
    let should_break = Arc::new(AtomicBool::new(false));
    let should_break_clone = should_break.clone();

    let handle = AppHandleManager.get_handle().unwrap();
    let id = handle.listen("break_exec", move |_| {
      should_break_clone.store(true, Ordering::Release);
    });

    // Main loop with optimized condition checking
    while !should_break.load(Ordering::Acquire) {
      match self.eval_expr(condition.clone())? {
        Value::Boolean(false) => break,
        Value::Boolean(true) => {
          // Execute the body first
          self.eval_block(body.clone())?;

          // Handle the update statement
          self.eval_stmt(update.clone())?;
        }
        _ => {
          handle.unlisten(id);
          return Err(InterpreterError::EvalError(
            self.location,
            "Condição do loop deve ser booleana".into(),
          ));
        }
      }
    }

    // Cleanup
    handle.unlisten(id);
    *self.environment.symbols.write() = state;

    Ok(Value::None)
  }
}
