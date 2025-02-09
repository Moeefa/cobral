mod environment;
mod error;
mod grammar;

use crate::shared::ast::{Expression, Statement};
use environment::Environment;
use error::ParserError;

use crate::lexer::token::{LabeledToken, Token};

#[allow(dead_code)]
pub struct Parser {
  tokens: Vec<LabeledToken>,
  env: Environment,
  pub current_token: LabeledToken,
}

impl Parser {
  pub fn new(tokens: Vec<LabeledToken>) -> Result<Vec<Statement>, ParserError> {
    let mut parser = Parser {
      tokens: tokens.clone(),
      env: Environment::new(),
      current_token: tokens[0].clone(),
    };

    parser.run()
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

  fn eat(&mut self, token: Token) -> Result<(), ParserError> {
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

  fn try_eat(&mut self, token: Token) -> Result<(), ParserError> {
    if &self.current_token.token == &token {
      self.next_token(); // Consume semicolon
    }

    Ok(())
  }

  fn next_token(&mut self) {
    self.tokens.remove(0);
    self.current_token = self.tokens[0].clone();
  }

  fn peek_token(&mut self) -> LabeledToken {
    self.tokens[1].clone()
  }

  // Modified block parsing method
  pub fn parse_block(&mut self) -> Result<Vec<Statement>, ParserError> {
    self.eat(Token::BraceL)?;

    let mut items = Vec::new();
    while self.current_token.token != Token::BraceR && self.current_token.token != Token::EOF {
      let item = self.parse_statement()?;
      items.push(item);
    }

    self.eat(Token::BraceR)?;
    Ok(items)
  }

  pub fn parse_arguments(&mut self) -> Result<Vec<Expression>, ParserError> {
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
