use crate::{
  interpreter::{error::InterpreterError, value::Value, Interpreter},
  shared::ast::{Expression, Statement},
};

impl Interpreter {
  pub fn eval_if_stmt(
    &mut self,
    condition: Option<Expression>,
    true_block: Vec<Statement>,
    else_if_block: Vec<(Box<Option<Expression>>, Vec<Statement>)>,
    else_block: Option<Vec<Statement>>,
  ) -> Result<Value, InterpreterError> {
    let condition = self.eval_expr(&condition.expect("Condição não encontrada"))?;

    // Ensure the condition result is a boolean
    let condition = match condition {
      Value::Boolean(b) => b,
      _ => {
        return Err(InterpreterError::EvalError(
          self.location,
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
        let else_if_condition = self.eval_expr(&else_if_condition.unwrap())?;

        let else_if_condition = match else_if_condition {
          Value::Boolean(b) => b,
          _ => {
            return Err(InterpreterError::EvalError(
              self.location,
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
        Ok(Value::None)
      }
    }
  }
}
