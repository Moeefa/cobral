use tauri::Listener;

use crate::{
  interpreter::{error::InterpreterError, value::Value, Interpreter},
  shared::ast::{Expression, Statement},
  shared::AppHandleManager,
};

impl Interpreter {
  pub fn eval_while_stmt(
    &mut self,
    condition: Expression,
    body: Vec<Statement>,
  ) -> Result<Value, InterpreterError> {
    // Set up break detection
    let should_break = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let should_break_clone = should_break.clone();

    // Listen for break_exec event
    let handle = AppHandleManager.get_handle().unwrap();
    let id = handle.listen("break_exec", move |_| {
      should_break_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    });

    // Main loop
    while !should_break.load(std::sync::atomic::Ordering::SeqCst) {
      match self.eval_expr(&condition)? {
        Value::Boolean(false) => break,
        Value::Boolean(true) => (),
        _ => {
          handle.unlisten(id);
          return Err(InterpreterError::EvalError(
            self.location,
            "Condição do loop deve ser booleana".into(),
          ));
        }
      }

      self.eval_block(&body)?;
    }

    handle.unlisten(id);

    Ok(Value::None)
  }
}
