use crate::{
  interpreter::{error::InterpreterError, value::Value, Interpreter},
  shared::ast::Expression,
};

impl Interpreter {
  pub fn eval_postfix_decrement_expr(&self, expr: Expression) -> Result<Value, InterpreterError> {
    if let Expression::Identifier(name, _) = expr {
      // First get the original value while holding a read lock
      let original_value = {
        let symbol = self.environment.get_symbol(&name).ok_or_else(|| {
          InterpreterError::EvalError(self.location, format!("Símbolo '{}' não definido", name))
        })?;

        let value = symbol.read().get_value().clone();

        value
      };

      let new_value = match &original_value {
        Value::Integer(n) => Value::Integer(n + 1),
        Value::Float(n) => Value::Float(n + 1.0),
        _ => {
          return Err(InterpreterError::EvalError(
            self.location,
            "Operador de decremento deve ser aplicado a um valor numérico".to_string(),
          ))
        }
      };

      // Set the new value using our new method
      self
        .environment
        .set_symbol_value(&name, new_value)
        .map_err(|e| InterpreterError::EvalError(self.location, e.to_string()))?;

      Ok(original_value)
    } else {
      Err(InterpreterError::EvalError(
        self.location,
        "Operador de decremento deve ser aplicado a um símbolo".to_string(),
      ))
    }
  }

  pub fn eval_postfix_increment_expr(&self, expr: Expression) -> Result<Value, InterpreterError> {
    if let Expression::Identifier(name, _) = expr {
      // First get the original value while holding a read lock
      let original_value = {
        let symbol = self.environment.get_symbol(&name).ok_or_else(|| {
          InterpreterError::EvalError(self.location, format!("Símbolo '{}' não definido", name))
        })?;

        let value = symbol.read().get_value().clone();

        value
      };

      let new_value = match &original_value {
        Value::Integer(n) => Value::Integer(n + 1),
        Value::Float(n) => Value::Float(n + 1.0),
        _ => {
          return Err(InterpreterError::EvalError(
            self.location,
            "Operador de incremento deve ser aplicado a um valor numérico".to_string(),
          ))
        }
      };

      // Set the new value using our new method
      self
        .environment
        .set_symbol_value(&name, new_value)
        .map_err(|e| InterpreterError::EvalError(self.location, e.to_string()))?;

      Ok(original_value)
    } else {
      Err(InterpreterError::EvalError(
        self.location,
        "Operador de incremento deve ser aplicado a um símbolo".to_string(),
      ))
    }
  }
}
