use crate::{
  interpreter::{error::InterpreterError, value::Value, Interpreter},
  lexer::token::Token,
  shared::ast::Expression,
};

impl Interpreter {
  pub fn eval_arithmetic_expr(
    &mut self,
    lhs: Expression,
    op: Token,
    rhs: Expression,
  ) -> Result<Value, InterpreterError> {
    let lhs_value = self.eval_expr(&lhs)?;

    let rhs_value = self.eval_expr(&rhs)?;

    match (lhs_value, rhs_value, op) {
      // Handle integer arithmetic
      (Value::Integer(l), Value::Integer(r), Token::Rem) => Ok(Value::Integer(l % r)),
      (Value::Integer(l), Value::Integer(r), Token::Plus) => Ok(Value::Integer(l + r)),
      (Value::Integer(l), Value::Integer(r), Token::Minus) => Ok(Value::Integer(l - r)),
      (Value::Integer(l), Value::Integer(r), Token::Asterisk) => Ok(Value::Integer(l * r)),
      (Value::Integer(l), Value::Integer(r), Token::Slash) => {
        if r == 0 {
          Err(InterpreterError::EvalError(
            self.location.clone(),
            "Divisão por zero".to_string(),
          ))
        } else {
          Ok(Value::Integer(l / r))
        }
      }

      // Handle float arithmetic
      (Value::Float(l), Value::Float(r), Token::Rem) => Ok(Value::Float(l % r)),
      (Value::Float(l), Value::Float(r), Token::Plus) => Ok(Value::Float(l + r)),
      (Value::Float(l), Value::Float(r), Token::Minus) => Ok(Value::Float(l - r)),
      (Value::Float(l), Value::Float(r), Token::Asterisk) => Ok(Value::Float(l * r)),
      (Value::Float(l), Value::Float(r), Token::Slash) => {
        if r == 0.0 {
          Err(InterpreterError::EvalError(
            self.location.clone(),
            "Divisão por zero".to_string(),
          ))
        } else {
          Ok(Value::Float(l / r))
        }
      }

      // Mixed type arithmetic (integer and float)
      (Value::Integer(l), Value::Float(r), Token::Rem) => Ok(Value::Float((l as f64) % r)),
      (Value::Integer(l), Value::Float(r), Token::Plus) => Ok(Value::Float((l as f64) + r)),
      (Value::Integer(l), Value::Float(r), Token::Minus) => Ok(Value::Float((l as f64) - r)),
      (Value::Integer(l), Value::Float(r), Token::Asterisk) => Ok(Value::Float((l as f64) * r)),
      (Value::Integer(l), Value::Float(r), Token::Slash) => {
        if r == 0.0 {
          Err(InterpreterError::EvalError(
            self.location.clone(),
            "Divisão por zero".to_string(),
          ))
        } else {
          Ok(Value::Float((l as f64) / r))
        }
      }
      (Value::Float(l), Value::Integer(r), Token::Rem) => Ok(Value::Float(l % (r as f64))),
      (Value::Float(l), Value::Integer(r), Token::Plus) => Ok(Value::Float(l + (r as f64))),
      (Value::Float(l), Value::Integer(r), Token::Minus) => Ok(Value::Float(l - (r as f64))),
      (Value::Float(l), Value::Integer(r), Token::Asterisk) => Ok(Value::Float(l * (r as f64))),
      (Value::Float(l), Value::Integer(r), Token::Slash) => {
        if r == 0 {
          Err(InterpreterError::EvalError(
            self.location.clone(),
            "Divisão por zero".to_string(),
          ))
        } else {
          Ok(Value::Float(l / (r as f64)))
        }
      }

      // String concatenation
      (Value::String(l), Value::String(r), Token::Plus) => Ok(Value::String(format!("{}{}", l, r))),
      (Value::String(l), Value::Integer(r), Token::Plus) => {
        Ok(Value::String(format!("{}{}", l, r)))
      }
      (Value::String(l), Value::Float(r), Token::Plus) => Ok(Value::String(format!("{}{}", l, r))),
      (Value::Integer(l), Value::String(r), Token::Plus) => {
        Ok(Value::String(format!("{}{}", l, r)))
      }
      (Value::Float(l), Value::String(r), Token::Plus) => Ok(Value::String(format!("{}{}", l, r))),
      (Value::String(l), Value::Boolean(r), Token::Plus) => {
        Ok(Value::String(format!("{}{}", l, r)))
      }
      (Value::Boolean(l), Value::String(r), Token::Plus) => {
        Ok(Value::String(format!("{}{}", l, r)))
      }
      (Value::String(l), Value::List(r), Token::Plus) => {
        Ok(Value::String(format!("{}{:#?}", l, r)))
      }
      (Value::List(l), Value::String(r), Token::Plus) => {
        Ok(Value::String(format!("{:#?}{}", l, r)))
      }
      (Value::String(l), Value::None, Token::Plus) => Ok(Value::String(format!("{}{}", l, ""))),
      (Value::None, Value::String(r), Token::Plus) => Ok(Value::String(format!("{}{}", "", r))),

      // Error for incompatible types
      _ => Err(InterpreterError::EvalError(
        self.location.clone(),
        "Operação inválida entre tipos incompatíveis".into(),
      )),
    }
  }
}

impl Interpreter {
  pub fn eval_unary_expr(
    &mut self,
    token: Token,
    expr: Expression,
  ) -> Result<Value, InterpreterError> {
    let value = self.eval_expr(&expr)?;

    match token {
      Token::Plus => match value {
        Value::Integer(n) => Ok(Value::Integer(n.abs())),
        Value::Float(n) => Ok(Value::Float(n.abs())),
        _ => Err(InterpreterError::EvalError(
          self.location.clone(),
          "Operador '+' deve ser aplicado a um valor numérico".to_string(),
        )),
      },
      Token::Minus => match value {
        Value::Integer(n) => Ok(Value::Integer(-n)),
        Value::Float(n) => Ok(Value::Float(-n)),
        _ => Err(InterpreterError::EvalError(
          self.location.clone(),
          "Operador '-' deve ser aplicado a um valor numérico".to_string(),
        )),
      },
      Token::Not => match value {
        Value::Boolean(b) => Ok(Value::Boolean(!b)),
        _ => Err(InterpreterError::EvalError(
          self.location.clone(),
          "Operador 'não' deve ser aplicado a um valor booleano".to_string(),
        )),
      },
      _ => Err(InterpreterError::EvalError(
        self.location.clone(),
        "Operador inválido".to_string(),
      )),
    }
  }
}
