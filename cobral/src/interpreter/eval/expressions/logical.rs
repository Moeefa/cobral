use crate::{
  interpreter::{error::InterpreterError, value::Value, Interpreter},
  lexer::token::Token,
  shared::ast::Expression,
};

impl Interpreter {
  pub fn eval_logical_expr(
    &mut self,
    lhs: Expression,
    op: Token,
    rhs: Expression,
  ) -> Result<Value, InterpreterError> {
    // Evaluate the left-hand side (LHS) expression
    let lhs_value = self.eval_expr(lhs)?;

    // Ensure the LHS is a boolean
    let lhs_bool = match lhs_value {
      Value::Boolean(b) => b,
      _ => {
        return Err(InterpreterError::EvalError(
          self.location,
          format!(
            "Operação lógica deve ser booleana, mas encontrou: {:?}",
            lhs_value
          ),
        ));
      }
    };

    // Evaluate the right-hand side (RHS) expression
    let rhs_value = self.eval_expr(rhs)?;

    // Ensure the RHS is a boolean
    let rhs_bool = match rhs_value {
      Value::Boolean(b) => b,
      _ => {
        return Err(InterpreterError::EvalError(
          self.location,
          format!(
            "Operação lógica deve ser booleana, mas encontrou: {:?}",
            rhs_value
          ),
        ));
      }
    };

    // Apply the logical operation (AND `e` or OR `ou`)
    match op {
      Token::And => Ok(Value::Boolean(lhs_bool && rhs_bool)),
      Token::Or => Ok(Value::Boolean(lhs_bool || rhs_bool)),
      _ => Err(InterpreterError::EvalError(
        self.location,
        "Operação lógica inválida".to_string(),
      )),
    }
  }
}
