use types::{Data, Expr, InterpreterError};

use crate::Interpreter;

impl Interpreter {
  pub fn eval_postfix_decrement(
    &self,
    expr: Box<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    if let Expr::Symbol(name) = *expr {
      let value = self.variables.lock().unwrap().get(&name).unwrap().clone();
      let new_value = match value {
        Data::Integer(n) => Data::Integer(n - 1),
        Data::Float(n) => Data::Float(n - 1.0),
        _ => {
          return Err(InterpreterError::ParseError(
            line,
            "Operador '--' deve ser aplicado a um valor numérico".to_string(),
          ))
        }
      };
      self
        .variables
        .lock()
        .unwrap()
        .insert(name, new_value.clone());
      Ok(new_value)
    } else {
      Err(InterpreterError::ParseError(
        line,
        "Operador '--' deve ser aplicado a um símbolo".to_string(),
      ))
    }
  }

  pub fn eval_postfix_increment(
    &self,
    expr: Box<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    if let Expr::Symbol(name) = *expr {
      let value = self.variables.lock().unwrap().get(&name).unwrap().clone();
      let new_value = match value {
        Data::Integer(n) => Data::Integer(n + 1),
        Data::Float(n) => Data::Float(n + 1.0),
        _ => {
          return Err(InterpreterError::ParseError(
            line,
            "Operador '++' deve ser aplicado a um valor numérico".to_string(),
          ))
        }
      };
      self
        .variables
        .lock()
        .unwrap()
        .insert(name, new_value.clone());
      Ok(new_value)
    } else {
      Err(InterpreterError::ParseError(
        line,
        "Operador '++' deve ser aplicado a um símbolo".to_string(),
      ))
    }
  }
}
