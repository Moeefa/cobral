use ::enums::{Data, Expr, LabeledExpr};

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_const(
    &self,
    name: String,
    value: Expr,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    if self.env.constants.read().contains_key(&name) {
      return Err(InterpreterError::ConstantRedeclarationError(
        line,
        name.clone(),
      ));
    }

    let value = self.eval(LabeledExpr {
      expr: value,
      line_number: line,
    })?;

    self.env.constants.write().insert(name, value.clone());

    Ok(value)
  }
}
