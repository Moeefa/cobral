use types::{Data, Expr, InterpreterError, LabeledExpr};

use crate::Interpreter;

impl Interpreter {
  pub fn eval_list(&self, elements: Vec<Expr>, line: usize) -> Result<Data, InterpreterError> {
    let mut evaluated_elements = Vec::new();
    for element in elements {
      let value = self.eval(LabeledExpr {
        expr: element,
        line_number: line,
      })?;
      evaluated_elements.push(value);
    }

    Ok(Data::List(evaluated_elements))
  }
}
