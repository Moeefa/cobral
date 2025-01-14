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
    if self.variables.lock().unwrap().contains_key(&name) {
      let value = self.eval(LabeledExpr {
        expr: value,
        line_number: line,
      })?;
      self
        .variables
        .lock()
        .unwrap()
        .insert(name.clone(), value.clone());
      Ok(value)
    } else {
      if self.constants.lock().unwrap().contains_key(&name) {
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
