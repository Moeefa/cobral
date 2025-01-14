use ::enums::{Data, Expr, LabeledExpr, Token};

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_comparison(
    &self,
    lhs: Expr,
    op: Token,
    rhs: Expr,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    // Evaluate left-hand side expression
    let lhs_value = self.eval(LabeledExpr {
      expr: lhs,
      line_number: line,
    })?;

    // Evaluate right-hand side expression
    let rhs_value = self.eval(LabeledExpr {
      expr: rhs,
      line_number: line,
    })?;

    match (lhs_value, rhs_value, op) {
      (Data::Integer(l), Data::Integer(r), Token::GreaterThan) => Ok(Data::Boolean(l > r)),
      (Data::Integer(l), Data::Integer(r), Token::GreaterThanEqual) => Ok(Data::Boolean(l >= r)),
      (Data::Integer(l), Data::Integer(r), Token::LessThan) => Ok(Data::Boolean(l < r)),
      (Data::Integer(l), Data::Integer(r), Token::LessThanEqual) => Ok(Data::Boolean(l <= r)),
      (Data::Integer(l), Data::Integer(r), Token::EqualEqual) => Ok(Data::Boolean(l == r)),
      (Data::Integer(l), Data::Integer(r), Token::NotEqual) => Ok(Data::Boolean(l != r)),

      (Data::Float(l), Data::Float(r), Token::GreaterThan) => Ok(Data::Boolean(l > r)),
      (Data::Float(l), Data::Float(r), Token::GreaterThanEqual) => Ok(Data::Boolean(l >= r)),
      (Data::Float(l), Data::Float(r), Token::LessThan) => Ok(Data::Boolean(l < r)),
      (Data::Float(l), Data::Float(r), Token::LessThanEqual) => Ok(Data::Boolean(l <= r)),
      (Data::Float(l), Data::Float(r), Token::EqualEqual) => Ok(Data::Boolean(l == r)),
      (Data::Float(l), Data::Float(r), Token::NotEqual) => Ok(Data::Boolean(l != r)),

      (Data::Float(l), Data::Integer(r), Token::GreaterThan) => Ok(Data::Boolean(l > (r as f64))),
      (Data::Float(l), Data::Integer(r), Token::GreaterThanEqual) => {
        Ok(Data::Boolean(l >= (r as f64)))
      }
      (Data::Float(l), Data::Integer(r), Token::LessThan) => Ok(Data::Boolean(l < (r as f64))),
      (Data::Float(l), Data::Integer(r), Token::LessThanEqual) => {
        Ok(Data::Boolean(l <= (r as f64)))
      }
      (Data::Float(l), Data::Integer(r), Token::EqualEqual) => Ok(Data::Boolean(l == (r as f64))),
      (Data::Float(l), Data::Integer(r), Token::NotEqual) => Ok(Data::Boolean(l != (r as f64))),

      (Data::Integer(l), Data::Float(r), Token::GreaterThan) => Ok(Data::Boolean((l as f64) > r)),
      (Data::Integer(l), Data::Float(r), Token::GreaterThanEqual) => {
        Ok(Data::Boolean((l as f64) >= r))
      }
      (Data::Integer(l), Data::Float(r), Token::LessThan) => Ok(Data::Boolean((l as f64) < r)),
      (Data::Integer(l), Data::Float(r), Token::LessThanEqual) => {
        Ok(Data::Boolean((l as f64) <= r))
      }
      (Data::Integer(l), Data::Float(r), Token::EqualEqual) => Ok(Data::Boolean((l as f64) == r)),
      (Data::Integer(l), Data::Float(r), Token::NotEqual) => Ok(Data::Boolean((l as f64) != r)),

      (Data::String(l), Data::String(r), Token::EqualEqual) => Ok(Data::Boolean(l == r)),
      (Data::String(l), Data::String(r), Token::NotEqual) => Ok(Data::Boolean(l != r)),
      (Data::String(l), Data::String(r), Token::GreaterThanEqual) => Ok(Data::Boolean(l >= r)),
      (Data::String(l), Data::String(r), Token::GreaterThan) => Ok(Data::Boolean(l > r)),
      (Data::String(l), Data::String(r), Token::LessThanEqual) => Ok(Data::Boolean(l <= r)),
      (Data::String(l), Data::String(r), Token::LessThan) => Ok(Data::Boolean(l < r)),

      (Data::Boolean(l), Data::Boolean(r), Token::EqualEqual) => Ok(Data::Boolean(l == r)),
      (Data::Boolean(l), Data::Boolean(r), Token::NotEqual) => Ok(Data::Boolean(l != r)),

      // Catch-all for invalid comparisons
      _ => Err(InterpreterError::EvalError(
        line,
        "Comparação não suportada".to_string(),
      )),
    }
  }
}
