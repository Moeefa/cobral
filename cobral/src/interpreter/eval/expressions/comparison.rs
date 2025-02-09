use crate::{
  interpreter::{error::InterpreterError, value::Value, Interpreter},
  lexer::token::Token,
  shared::ast::Expression,
};

impl Interpreter {
  pub fn eval_comparison_expr(
    &mut self,
    lhs: Expression,
    op: Token,
    rhs: Expression,
  ) -> Result<Value, InterpreterError> {
    // Evaluate left-hand side expression
    let lhs_value = self.eval_expr(lhs)?;

    // Evaluate right-hand side expression
    let rhs_value = self.eval_expr(rhs)?;

    match (lhs_value, rhs_value, op) {
      (Value::Integer(l), Value::Integer(r), Token::Greater) => Ok(Value::Boolean(l > r)),
      (Value::Integer(l), Value::Integer(r), Token::GreaterEquals) => Ok(Value::Boolean(l >= r)),
      (Value::Integer(l), Value::Integer(r), Token::Less) => Ok(Value::Boolean(l < r)),
      (Value::Integer(l), Value::Integer(r), Token::LessEquals) => Ok(Value::Boolean(l <= r)),
      (Value::Integer(l), Value::Integer(r), Token::Equals) => Ok(Value::Boolean(l == r)),
      (Value::Integer(l), Value::Integer(r), Token::NotEquals) => Ok(Value::Boolean(l != r)),

      (Value::Float(l), Value::Float(r), Token::Greater) => Ok(Value::Boolean(l > r)),
      (Value::Float(l), Value::Float(r), Token::GreaterEquals) => Ok(Value::Boolean(l >= r)),
      (Value::Float(l), Value::Float(r), Token::Less) => Ok(Value::Boolean(l < r)),
      (Value::Float(l), Value::Float(r), Token::LessEquals) => Ok(Value::Boolean(l <= r)),
      (Value::Float(l), Value::Float(r), Token::Equals) => Ok(Value::Boolean(l == r)),
      (Value::Float(l), Value::Float(r), Token::NotEquals) => Ok(Value::Boolean(l != r)),

      (Value::Float(l), Value::Integer(r), Token::Greater) => Ok(Value::Boolean(l > (r as f64))),
      (Value::Float(l), Value::Integer(r), Token::GreaterEquals) => {
        Ok(Value::Boolean(l >= (r as f64)))
      }
      (Value::Float(l), Value::Integer(r), Token::Less) => Ok(Value::Boolean(l < (r as f64))),
      (Value::Float(l), Value::Integer(r), Token::LessEquals) => {
        Ok(Value::Boolean(l <= (r as f64)))
      }
      (Value::Float(l), Value::Integer(r), Token::Equals) => Ok(Value::Boolean(l == (r as f64))),
      (Value::Float(l), Value::Integer(r), Token::NotEquals) => Ok(Value::Boolean(l != (r as f64))),

      (Value::Integer(l), Value::Float(r), Token::Greater) => Ok(Value::Boolean((l as f64) > r)),
      (Value::Integer(l), Value::Float(r), Token::GreaterEquals) => {
        Ok(Value::Boolean((l as f64) >= r))
      }
      (Value::Integer(l), Value::Float(r), Token::Less) => Ok(Value::Boolean((l as f64) < r)),
      (Value::Integer(l), Value::Float(r), Token::LessEquals) => {
        Ok(Value::Boolean((l as f64) <= r))
      }
      (Value::Integer(l), Value::Float(r), Token::Equals) => Ok(Value::Boolean((l as f64) == r)),
      (Value::Integer(l), Value::Float(r), Token::NotEquals) => Ok(Value::Boolean((l as f64) != r)),

      (Value::String(l), Value::String(r), Token::Equals) => Ok(Value::Boolean(l == r)),
      (Value::String(l), Value::String(r), Token::NotEquals) => Ok(Value::Boolean(l != r)),
      (Value::String(l), Value::String(r), Token::GreaterEquals) => Ok(Value::Boolean(l >= r)),
      (Value::String(l), Value::String(r), Token::Greater) => Ok(Value::Boolean(l > r)),
      (Value::String(l), Value::String(r), Token::LessEquals) => Ok(Value::Boolean(l <= r)),
      (Value::String(l), Value::String(r), Token::Less) => Ok(Value::Boolean(l < r)),

      (Value::Boolean(l), Value::Boolean(r), Token::Equals) => Ok(Value::Boolean(l == r)),
      (Value::Boolean(l), Value::Boolean(r), Token::NotEquals) => Ok(Value::Boolean(l != r)),

      // Catch-all for invalid comparisons
      _ => Err(InterpreterError::EvalError(
        self.location,
        "Comparação não suportada".into(),
      )),
    }
  }
}
