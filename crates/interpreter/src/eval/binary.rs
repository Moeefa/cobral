use ::enums::{Data, Expr, LabeledExpr, Token};

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_binary(
    &self,
    lhs: Expr,
    op: Token,
    rhs: Expr,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    let lhs_value = self.eval(LabeledExpr {
      expr: lhs,
      line_number: line,
    })?;

    let rhs_value = self.eval(LabeledExpr {
      expr: rhs,
      line_number: line,
    })?;

    match (lhs_value, rhs_value, op) {
      // Handle integer arithmetic
      (Data::Integer(l), Data::Integer(r), Token::Rem) => Ok(Data::Integer(l % r)),
      (Data::Integer(l), Data::Integer(r), Token::Plus) => Ok(Data::Integer(l + r)),
      (Data::Integer(l), Data::Integer(r), Token::Minus) => Ok(Data::Integer(l - r)),
      (Data::Integer(l), Data::Integer(r), Token::Times) => Ok(Data::Integer(l * r)),
      (Data::Integer(l), Data::Integer(r), Token::Divide) => {
        if r == 0 {
          Err(InterpreterError::EvalError(
            line,
            "Divisão por zero".to_string(),
          ))
        } else {
          Ok(Data::Integer(l / r))
        }
      }

      // Handle float arithmetic
      (Data::Float(l), Data::Float(r), Token::Rem) => Ok(Data::Float(l % r)),
      (Data::Float(l), Data::Float(r), Token::Plus) => Ok(Data::Float(l + r)),
      (Data::Float(l), Data::Float(r), Token::Minus) => Ok(Data::Float(l - r)),
      (Data::Float(l), Data::Float(r), Token::Times) => Ok(Data::Float(l * r)),
      (Data::Float(l), Data::Float(r), Token::Divide) => {
        if r == 0.0 {
          Err(InterpreterError::EvalError(
            line,
            "Divisão por zero".to_string(),
          ))
        } else {
          Ok(Data::Float(l / r))
        }
      }

      // Mixed type arithmetic (integer and float)
      (Data::Integer(l), Data::Float(r), Token::Rem) => Ok(Data::Float((l as f64) % r)),
      (Data::Integer(l), Data::Float(r), Token::Plus) => Ok(Data::Float((l as f64) + r)),
      (Data::Integer(l), Data::Float(r), Token::Minus) => Ok(Data::Float((l as f64) - r)),
      (Data::Integer(l), Data::Float(r), Token::Times) => Ok(Data::Float((l as f64) * r)),
      (Data::Integer(l), Data::Float(r), Token::Divide) => {
        if r == 0.0 {
          Err(InterpreterError::EvalError(
            line,
            "Divisão por zero".to_string(),
          ))
        } else {
          Ok(Data::Float((l as f64) / r))
        }
      }
      (Data::Float(l), Data::Integer(r), Token::Rem) => Ok(Data::Float(l % (r as f64))),
      (Data::Float(l), Data::Integer(r), Token::Plus) => Ok(Data::Float(l + (r as f64))),
      (Data::Float(l), Data::Integer(r), Token::Minus) => Ok(Data::Float(l - (r as f64))),
      (Data::Float(l), Data::Integer(r), Token::Times) => Ok(Data::Float(l * (r as f64))),
      (Data::Float(l), Data::Integer(r), Token::Divide) => {
        if r == 0 {
          Err(InterpreterError::EvalError(
            line,
            "Divisão por zero".to_string(),
          ))
        } else {
          Ok(Data::Float(l / (r as f64)))
        }
      }

      // String concatenation
      (Data::String(l), Data::String(r), Token::Plus) => Ok(Data::String(format!("{}{}", l, r))),
      (Data::String(l), Data::Integer(r), Token::Plus) => Ok(Data::String(format!("{}{}", l, r))),
      (Data::String(l), Data::Float(r), Token::Plus) => Ok(Data::String(format!("{}{}", l, r))),
      (Data::Integer(l), Data::String(r), Token::Plus) => Ok(Data::String(format!("{}{}", l, r))),
      (Data::Float(l), Data::String(r), Token::Plus) => Ok(Data::String(format!("{}{}", l, r))),
      (Data::String(l), Data::Boolean(r), Token::Plus) => Ok(Data::String(format!("{}{}", l, r))),
      (Data::Boolean(l), Data::String(r), Token::Plus) => Ok(Data::String(format!("{}{}", l, r))),
      (Data::String(l), Data::List(r), Token::Plus) => Ok(Data::String(format!("{}{:#?}", l, r))),
      (Data::List(l), Data::String(r), Token::Plus) => Ok(Data::String(format!("{:#?}{}", l, r))),
      (Data::String(l), Data::None, Token::Plus) => Ok(Data::String(format!("{}{}", l, ""))),
      (Data::None, Data::String(r), Token::Plus) => Ok(Data::String(format!("{}{}", "", r))),

      // Error for incompatible types
      _ => Err(InterpreterError::EvalError(
        line,
        "Operação inválida entre tipos incompatíveis".to_string(),
      )),
    }
  }
}
