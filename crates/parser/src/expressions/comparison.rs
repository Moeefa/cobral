use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  fn valid_comparison(&self, lhs: Expr, _op: Token, rhs: Expr) -> bool {
    match (self.expr_type(&lhs), self.expr_type(&rhs)) {
      (Some("inteiro"), Some("real")) => true,
      (Some("real"), Some("inteiro")) => true,
      (Some(t1), Some(t2)) if t1 == t2 => true,
      _ => false, // Incompatible or unknown
    }
  }

  pub fn parse_comparison_expression(&mut self) -> Result<Option<Expr>, ParseError> {
    // Parse the left-hand side of the comparison
    let lhs = self.parse_primary_expression()?;

    if !matches!(
      self.current_token.token,
      Token::EqualEqual
        | Token::NotEqual
        | Token::LessThan
        | Token::GreaterThan
        | Token::LessThanEqual
        | Token::GreaterThanEqual
    ) {
      return Ok(lhs);
    }

    let op = self.current_token.token.clone();
    self.eat(op.clone())?; // Consume the operator

    // Parse the right-hand side (RHS) of the comparison
    let rhs = self.parse_primary_expression()?;

    if let Some(lhs) = lhs {
      if let Some(rhs) = rhs {
        if !self.valid_comparison(lhs.clone(), op.clone(), rhs.clone()) {
          return Err(ParseError::InvalidExpression(
            self.current_token.line_number,
            format!(
              "Comparação incompatível: '{}' ({}) e '{}' ({}) não podem ser comparados.",
              lhs,
              self
                .expr_type(&lhs)
                .or_else(|| Some("desconhecido"))
                .unwrap(),
              rhs,
              self
                .expr_type(&rhs)
                .or_else(|| Some("desconhecido"))
                .unwrap(),
            ),
          ));
        }

        return Ok(Some(Expr::Comparison(
          Box::new(lhs),
          op.clone(),
          Box::new(rhs),
        )));
      } else {
        return Err(ParseError::InvalidExpression(
          self.current_token.line_number,
          "Falta o lado direito da expressão de comparação".to_string(),
        ));
      }
    } else {
      return Err(ParseError::InvalidExpression(
        self.current_token.line_number,
        "Falta o lado esquerdo da expressão de comparação".to_string(),
      ));
    }
  }
}
