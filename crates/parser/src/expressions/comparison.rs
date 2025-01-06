use ::enums::{Expr, Token};

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  fn valid_comparison(&self, lhs: Expr, _op: Token, rhs: Expr) -> bool {
    match (self.expr_type(&lhs), self.expr_type(&rhs)) {
      (Some("inteiro"), Some("real")) => true,
      (Some("real"), Some("inteiro")) => true,
      (Some("desconhecido"), Some(_t2)) => true,
      (Some(_t1), Some("desconhecido")) => true,
      (Some(t1), Some(t2)) if t1 == t2 => true,
      _ => false, // Incompatible or unknown
    }
  }

  pub fn parse_comparison_expression(&mut self) -> Result<Option<Expr>, ParserError> {
    let lhs = self.parse_primary_expression()?; // Parse the left-hand side

    if !matches!(
      self.current_token.token,
      Token::EqualEqual
        | Token::NotEqual
        | Token::LessThan
        | Token::GreaterThan
        | Token::LessThanEqual
        | Token::GreaterThanEqual
    ) {
      return Ok(lhs); // Return if there's no comparison operator
    }

    let op = self.current_token.token.clone();
    self.eat(op.clone())?; // Consume the operator

    let rhs = self.parse_primary_expression()?; // Parse the right-hand side

    if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
      if !self.valid_comparison(lhs.clone(), op.clone(), rhs.clone()) {
        return Err(ParserError::InvalidExpression(
          self.current_token.line_number,
          format!(
            "Comparação incompatível: '{}' ({}) e '{}' ({}) não podem ser comparados.",
            lhs,
            self.expr_type(&lhs).unwrap_or("desconhecido"),
            rhs,
            self.expr_type(&rhs).unwrap_or("desconhecido"),
          ),
        ));
      }

      return Ok(Some(Expr::Comparison(Box::new(lhs), op, Box::new(rhs))));
    }

    Err(ParserError::InvalidExpression(
      self.current_token.line_number,
      "Faltam operandos para a expressão de comparação".to_string(),
    ))
  }
}
