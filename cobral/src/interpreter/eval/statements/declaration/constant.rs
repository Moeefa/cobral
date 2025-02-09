use crate::{
  interpreter::{error::InterpreterError, value::Value, Interpreter},
  shared::ast::Expression,
};

impl Interpreter {
  pub fn eval_constant_stmt(
    &mut self,
    name: String,
    value: Expression,
  ) -> Result<Value, InterpreterError> {
    let value = self.eval_expr(value)?;

    self.environment.define_constant(name, value.clone())?;

    Ok(value)
  }
}
