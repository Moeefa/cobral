use ::enums::{Data, Expr, LabeledExpr};

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_index(
    &self,
    name: String,
    value: Box<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    let index = self.eval(LabeledExpr {
      expr: *value,
      line_number: line,
    })?;

    let data = self.variables.lock().unwrap().get(&name).unwrap().clone();

    match data {
      Data::List(list) => {
        let index = match index {
          Data::Integer(i) => i as usize,
          _ => {
            return Err(InterpreterError::ExpressionEvaluationFailure(
              line,
              "Índice deve ser um número inteiro".to_string(),
            ))
          }
        };

        if index >= list.len() {
          return Err(InterpreterError::ExpressionEvaluationFailure(
            line,
            "Índice fora de alcance".to_string(),
          ));
        }

        Ok(list[index].clone())
      }
      _ => Err(InterpreterError::ExpressionEvaluationFailure(
        line,
        "A indexação é suportada somente em vetores".to_string(),
      )),
    }
  }
}
