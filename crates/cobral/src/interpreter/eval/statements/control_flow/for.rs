use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::event::GLOBAL_EVENT_SYSTEM;
use crate::interpreter::error::InterpreterError;
use crate::interpreter::value::Value;
use crate::interpreter::Interpreter;
use crate::shared::ast::{Expression, Statement};

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
    self.eval_stmt(&initializer)?;

    // Use a local AtomicBool for break detection
    let should_break = Arc::new(AtomicBool::new(false));
    let should_break_clone = should_break.clone();

    let id = GLOBAL_EVENT_SYSTEM.listen(
      "break_exec",
      Box::new(move |_| {
        should_break_clone.store(true, Ordering::SeqCst);
      }),
    );

    // Main loop with optimized condition checking
    while !should_break.load(Ordering::SeqCst) {
      match self.eval_expr(&condition)? {
        Value::Boolean(false) => break,
        Value::Boolean(true) => {
          // Execute the body first
          self.eval_block(&body)?;

          // Handle the update statement
          self.eval_stmt(&update)?;
        }
        _ => {
          GLOBAL_EVENT_SYSTEM.unlisten(id);
          *self.environment.symbols.write() = state;
          return Err(InterpreterError::EvalError(
            self.location.clone(),
            "Condição de laço inválida".into(),
          ));
        }
      }
    }

    // Cleanup
    GLOBAL_EVENT_SYSTEM.unlisten(id);
    *self.environment.symbols.write() = state;

    Ok(Value::None)
  }
}
