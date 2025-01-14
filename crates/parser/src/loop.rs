use ::enums::{Expr, Token};

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_for(&mut self) -> Result<Option<Expr>, ParserError> {
    self.eat(Token::For)?; // Consume "para"
    self.eat(Token::ParenL)?; // Consume the left parenthesis

    // Parse the initializer (e.g., i = 0)
    self.eat(Token::Let)?; // Consume `let`
    let name = match &self.current_token.token {
      Token::Symbol(ref name) => name.clone(),
      _ => {
        return Err(ParserError::ExpectedVariableName(
          self.current_token.clone(),
        ))
      }
    };
    self.next_token(); // Consume variable name
    self.eat(Token::Equals)?; // Consume `=`
    let expr = self.parse_expression().unwrap();
    self
      .context
      .variables
      .lock()
      .unwrap()
      .insert(name.clone(), Some(expr.clone().unwrap()));
    let initializer = Some(Expr::Let(name, Box::new(expr.unwrap())));
    self.eat(Token::Semicolon)?; // Consume the first semicolon
    if !matches!(initializer, Some(Expr::Let(_, _) | Expr::Assignment(_, _))) {
      return Err(ParserError::InvalidExpression(
        self.current_token.line_number,
        "Era esperado a inicialização de uma variável".to_string(),
      ));
    }

    // Parse the condition (e.g., i < 10)
    let condition = self.parse_comparison_expression()?;
    self.eat(Token::Semicolon)?; // Consume the second semicolon
    if !matches!(condition, Some(Expr::Comparison(_, _, _))) {
      return Err(ParserError::InvalidExpression(
        self.current_token.line_number,
        "Era esperado uma expressão de comparação".to_string(),
      ));
    }

    // Parse the update expression (e.g., i = i + 1)
    let update = match &self.current_token.token {
      Token::Let => self.parse_let(),
      Token::Increment | Token::Decrement | Token::Symbol(_) => {
        self.parse_expression().map_err(|e| e)
      }
      _ => Err(ParserError::UnexpectedToken(self.current_token.clone())),
    }?;
    if !matches!(
      update,
      Some(
        Expr::Assignment(_, _)
          | Expr::PostfixDecrement(_)
          | Expr::PostfixIncrement(_)
          | Expr::PrefixDecrement(_)
          | Expr::PrefixIncrement(_)
          | Expr::Let(_, _)
      )
    ) {
      return Err(ParserError::InvalidExpression(
        self.current_token.line_number,
        "Era esperado a atualização de uma variável".to_string(),
      ));
    }

    self.eat(Token::ParenR)?; // Consume the right parenthesis

    // Parse the block of code inside the loop
    let body = self.parse_block()?;

    self.try_eat(Token::Semicolon)?; // Optionally consume a semicolon after the block

    Ok(Some(Expr::For(
      Box::new(initializer.unwrap()),
      Box::new(condition.unwrap()),
      Box::new(update.unwrap()),
      body,
    )))
  }
}
