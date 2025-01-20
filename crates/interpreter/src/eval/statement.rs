use ::enums::{Data, Expr, LabeledExpr};

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_statement(
    &self,
    condition: Option<Expr>,
    true_block: Vec<Expr>,
    else_if_block: Vec<(Box<Option<Expr>>, Vec<Expr>)>,
    else_block: Option<Vec<Expr>>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    let condition = self.eval(LabeledExpr {
      expr: condition.expect("Condição não encontrada"),
      line_number: line,
    })?;

    // Ensure the condition result is a boolean
    let condition = match condition {
      Data::Boolean(b) => b,
      _ => {
        return Err(InterpreterError::ParserError(
          line,
          "Condição deve ser verdadeiro ou falso".to_string(),
        ))
      }
    };

    if condition {
      // Evaluate the true block if the condition is true
      self.eval_block(&true_block)
    } else {
      // Check each 'else if' block
      for (else_if_condition, else_if_block) in else_if_block {
        let else_if_condition = self.eval(LabeledExpr {
          expr: else_if_condition.unwrap(),
          line_number: line,
        })?;

        let else_if_condition = match else_if_condition {
          Data::Boolean(b) => b,
          _ => {
            return Err(InterpreterError::ParserError(
              line,
              "Condição em um 'senao se' deve ser verdadeiro ou falso".to_string(),
            ))
          }
        };

        if else_if_condition {
          return self.eval_block(&else_if_block);
        }
      }

      // If none of the 'else if' conditions are true, evaluate the 'else' block
      if let Some(else_block) = else_block {
        self.eval_block(&else_block)
      } else {
        Ok(Data::None)
      }
    }
  }
}
