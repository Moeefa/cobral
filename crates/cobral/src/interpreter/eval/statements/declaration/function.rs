use crate::{
  interpreter::{error::InterpreterError, value::Value, Interpreter},
  shared::ast::Statement,
};

impl Interpreter {
  pub fn eval_function_stmt(
    &self,
    name: String,
    args: Vec<String>,
    body: Vec<Statement>,
  ) -> Result<Value, InterpreterError> {
    self
      .environment
      .functions
      .write()
      .insert(name, (args, body));

    Ok(Value::None)
  }
}
