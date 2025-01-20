use ::enums::{Data, Expr, LabeledExpr};

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_let(&self, name: String, value: Expr, line: usize) -> Result<Data, InterpreterError> {
    let value = self.eval(LabeledExpr {
      expr: value,
      line_number: line,
    })?;

    self
      .env
      .variables
      .write()
      .insert(name.clone(), value.clone());

    Ok(value)
  }
}
