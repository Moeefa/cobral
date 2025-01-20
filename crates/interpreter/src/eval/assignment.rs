use ::enums::{Data, Expr, LabeledExpr};

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_assignment(
    &self,
    name: String,
    value: Expr,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    // Ensure the variable has been declared before reassigning
    if self.env.variables.read().contains_key(&name) {
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
    } else {
      if self.env.constants.read().contains_key(&name) {
        return Err(InterpreterError::ConstantRedeclarationError(
          line,
          name.clone(),
        ));
      }

      Err(InterpreterError::EvalError(
        line,
        format!("Variável desconhecida: {}", value),
      ))
    }
  }
}
