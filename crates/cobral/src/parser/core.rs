use super::environment::Environment;
use super::error::ParserError;
use super::utils::ParserUtils;
use crate::lexer::token::{LabeledToken, Token};
use crate::shared::ast::Statement;

#[allow(dead_code)]
pub struct Parser {
  tokens: Vec<LabeledToken>,
  pub env: Environment,
  pub current_token: LabeledToken,
}

impl Parser {
  pub fn new(tokens: Vec<LabeledToken>) -> Result<Vec<Statement>, ParserError> {
    Parser {
      tokens: tokens.clone(),
      env: Environment::new(),
      current_token: tokens[0].clone(),
    }
    .run()
  }

  fn run(&mut self) -> Result<Vec<Statement>, ParserError> {
    let mut items = Vec::new();

    while self.current_token.token != Token::EOF {
      let item = self.parse_statement()?;
      items.push(item);
      self.try_eat(Token::Semicolon)?;
    }

    Ok(items)
  }

  pub fn eat(&mut self, token: Token) -> Result<(), ParserError> {
    if &self.current_token.token == &token {
      self.next_token(); // Move to the next token
      Ok(())
    } else {
      Err(ParserError::ExpectedToken(
        self.current_token.clone(),
        token,
      ))
    }
  }

  pub fn try_eat(&mut self, token: Token) -> Result<(), ParserError> {
    if &self.current_token.token == &token {
      self.next_token(); // Consume token
    }

    Ok(())
  }

  pub fn next_token(&mut self) {
    self.tokens.remove(0);
    self.current_token = self.tokens[0].clone();
  }

  pub fn peek_token(&mut self) -> LabeledToken {
    self.tokens[1].clone()
  }
}

// Implement ParserUtils trait for Parser
impl ParserUtils for Parser {
  fn current_token(&self) -> &LabeledToken {
    &self.current_token
  }

  fn advance_token(&mut self) {
    
  }
}
