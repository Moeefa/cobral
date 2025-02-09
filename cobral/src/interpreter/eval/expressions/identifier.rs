use crate::interpreter::{error::InterpreterError, value::Value, Interpreter};

impl Interpreter {
  pub fn eval_identifier_expr(&self, identifier: String) -> Result<Value, InterpreterError> {
    if let Some(symbol_lock) = self.environment.get_symbol(&identifier) {
      let symbol = symbol_lock.write();
      Ok(symbol.get_value().clone())
    } else {
      Err(InterpreterError::EvalError(
        self.location,
        format!("Símbolo '{}' não definido", identifier),
      ))
    }
  }
}
