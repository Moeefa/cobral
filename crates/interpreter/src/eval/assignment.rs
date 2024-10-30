use types::{Data, Expr, InterpreterError, LabeledExpr};

use crate::Interpreter;

impl Interpreter {
  pub fn eval_assignment(
    &self,
    name: String,
    value: Box<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    // Ensure the variable has been declared before reassigning
    if self.variables.lock().unwrap().contains_key(&name) {
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
    } else {
      Err(InterpreterError::EvalError(
        line,
        format!("Variável desconhecida: {}", value),
      ))
    }
  }
}
