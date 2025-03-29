use crate::lexer::token::{LabeledToken, Token};

/// Common utilities for parser components
pub trait ParserUtils {
  fn current_token(&self) -> &LabeledToken;
  #[allow(dead_code)]
  fn advance_token(&mut self);

  fn check_terminator(&self) -> bool {
    // Return true if the current token is a terminator
    match self.current_token().token {
      Token::Semicolon | Token::EOF | Token::BraceR => true,
      _ => false,
    }
  }
}
