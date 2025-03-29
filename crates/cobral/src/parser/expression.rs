use super::core::Parser;
use super::error::ParserError;
use crate::lexer::token::Token;
use crate::shared::ast::Expression;

// Trait for expression parsing capabilities
pub trait ExpressionParser {
  fn parse_arguments(&mut self) -> Result<Vec<Expression>, ParserError>;
}

impl ExpressionParser for Parser {
  fn parse_arguments(&mut self) -> Result<Vec<Expression>, ParserError> {
    let mut args = Vec::new();
    let mut first_argument = true;

    while self.current_token.token != Token::ParenR && self.current_token.token != Token::EOF {
      if !first_argument {
        if self.current_token.token == Token::Comma {
          self.eat(Token::Comma)?;
        } else {
          return Err(ParserError::UnexpectedToken(self.current_token.clone()));
        }
      }

      match self.parse_expression() {
        Ok(expr) => args.push(expr),
        Err(e) => return Err(e),
      }

      first_argument = false;
    }

    if self.current_token.token == Token::ParenR {
      self.eat(Token::ParenR)?;
    } else {
      return Err(ParserError::UnexpectedToken(self.current_token.clone()));
    }

    Ok(args)
  }
}
