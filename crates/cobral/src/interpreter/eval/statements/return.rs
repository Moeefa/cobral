use crate::{
  interpreter::{error::InterpreterError, value::Value, Interpreter},
  shared::ast::{Expression, Location},
};

impl Interpreter {
  pub fn eval_return_stmt(
    &mut self,
    value: Option<Box<Expression>>,
    location: Location,
  ) -> Result<Value, InterpreterError> {
    // Check if we are inside a function scope
    if !self.environment.is_in_function_scope() {
      return Err(InterpreterError::EvalError(
        location,
        "Comando 'retorne' só pode ser usado dentro de funções".to_string(),
      ));
    }

    let return_value = if let Some(value) = value {
      self.eval_expr(&*value)?
    } else {
      // Default to 0 instead of None when no return value is provided
      Value::Integer(0)
    };

    // Wrap the value in Return
    Ok(Value::Return(Box::new(return_value)))
  }
}
