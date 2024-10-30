use types::{Data, Expr, InterpreterError, LabeledExpr};

use crate::Interpreter;

impl Interpreter {
  pub fn eval_let(
    &self,
    name: String,
    value: Box<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    let value = self.eval(LabeledExpr {
      expr: *value,
      line_number: line,
    })?;

    self
      .variables
      .lock()
      .unwrap()
      .insert(name.clone(), value.clone());

    Ok(value)
  }
}
