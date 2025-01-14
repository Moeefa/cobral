use ::enums::{Expr, Token};

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_while(&mut self) -> Result<Option<Expr>, ParserError> {
    self.eat(Token::While)?; // Consume "enquanto"
    self.eat(Token::ParenL)?; // Consume the left parenthesis

    // Parse the condition (e.g., i < 10)
    let condition = self.parse_comparison_expression()?;
    if !matches!(condition, Some(Expr::Comparison(_, _, _))) {
      return Err(ParserError::InvalidExpression(
        self.current_token.line_number,
        "Era esperado uma expressão de comparação".to_string(),
      ));
    }

    self.eat(Token::ParenR)?; // Consume the right parenthesis

    // Parse the block of code inside the loop
    let body = self.parse_block()?;

    self.try_eat(Token::Semicolon)?; // Optionally consume a semicolon after the block

    Ok(Some(Expr::While(Box::new(condition.unwrap()), body)))
  }
}
