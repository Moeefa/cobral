use types::{Data, Expr, InterpreterError, LabeledExpr};

use crate::Interpreter;

impl Interpreter {
  pub fn eval_const(
    &self,
    name: String,
    value: Box<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    if self.constants.lock().unwrap().contains_key(&name) {
      logger::error(InterpreterError::ConstantRedeclarationError(
        line,
        name.clone(),
      ));
    }

    let value = self.eval(LabeledExpr {
      expr: *value,
      line_number: line,
    })?;

    self.constants.lock().unwrap().insert(name, value.clone());

    Ok(value)
  }
}
