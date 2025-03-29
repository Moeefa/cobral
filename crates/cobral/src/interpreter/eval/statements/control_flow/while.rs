use crate::{
  event::GLOBAL_EVENT_SYSTEM,
  interpreter::{error::InterpreterError, value::Value, Interpreter},
  shared::ast::{Expression, Statement},
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
    let id = GLOBAL_EVENT_SYSTEM.listen(
      "break_exec",
      Box::new(move |_| {
        should_break_clone.store(true, std::sync::atomic::Ordering::SeqCst);
      }),
    );

    // Main loop
    while !should_break.load(std::sync::atomic::Ordering::SeqCst) {
      match self.eval_expr(&condition)? {
        Value::Boolean(false) => break,
        Value::Boolean(true) => (),
        _ => {
          GLOBAL_EVENT_SYSTEM.unlisten(id);
          return Err(InterpreterError::EvalError(
            self.location.clone(),
            "Condição do loop deve ser booleana".into(),
          ));
        }
      }

      self.eval_block(&body)?;
    }

    GLOBAL_EVENT_SYSTEM.unlisten(id);

    Ok(Value::None)
  }
}
