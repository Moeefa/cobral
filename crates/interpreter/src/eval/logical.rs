use ::enums::{Data, Expr, LabeledExpr, Token};

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_logical(
    &self,
    lhs: Box<Expr>,
    op: Token,
    rhs: Box<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    // Evaluate the left-hand side (LHS) expression
    let lhs_value = self.eval(LabeledExpr {
      expr: *lhs,
      line_number: line,
    })?;

    // Ensure the LHS is a boolean
    let lhs_bool = match lhs_value {
      Data::Boolean(b) => b,
      _ => {
        return Err(InterpreterError::ParserError(
          line,
          format!(
            "Operação lógica deve ser booleana, mas encontrou: {:?}",
            lhs_value
          ),
        ));
      }
    };

    // Evaluate the right-hand side (RHS) expression
    let rhs_value = self.eval(LabeledExpr {
      expr: *rhs,
      line_number: line,
    })?;

    // Ensure the RHS is a boolean
    let rhs_bool = match rhs_value {
      Data::Boolean(b) => b,
      _ => {
        return Err(InterpreterError::ParserError(
          line,
          format!(
            "Operação lógica deve ser booleana, mas encontrou: {:?}",
            rhs_value
          ),
        ));
      }
    };

    // Apply the logical operation (AND `e` or OR `ou`)
    match op {
      Token::And => Ok(Data::Boolean(lhs_bool && rhs_bool)),
      Token::Or => Ok(Data::Boolean(lhs_bool || rhs_bool)),
      _ => Err(InterpreterError::ParserError(
        line,
        "Operação lógica inválida".to_string(),
      )),
    }
  }
}
