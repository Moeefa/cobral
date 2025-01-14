use ::enums::{Data, Expr, LabeledExpr};
use enums::Token;

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_unary(
    &self,
    token: Token,
    expr: Expr,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    let value = self.eval(LabeledExpr {
      expr: expr,
      line_number: line,
    })?;

    match token {
      Token::Minus => match value {
        Data::Integer(n) => Ok(Data::Integer(-n)),
        Data::Float(n) => Ok(Data::Float(-n)),
        _ => Err(InterpreterError::ParserError(
          line,
          "Operador '-' deve ser aplicado a um valor numérico".to_string(),
        )),
      },
      Token::Not => match value {
        Data::Boolean(b) => Ok(Data::Boolean(!b)),
        _ => Err(InterpreterError::ParserError(
          line,
          "Operador 'não' deve ser aplicado a um valor booleano".to_string(),
        )),
      },
      _ => Err(InterpreterError::ParserError(
        line,
        "Operador inválido".to_string(),
      )),
    }
  }
}
