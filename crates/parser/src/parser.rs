mod argument;
mod r#const;
mod expressions;
mod function;
mod import;
mod r#let;
mod list;
mod r#loop;
mod r#return;
mod statement;

use std::collections::HashMap;

use lexer::Lexer;
use libs::load_libs;
use types::{Data, Expr, LabeledToken, ParseError, Token};

#[allow(dead_code)]
pub struct Parser<'a> {
  lexer: Lexer<'a>,
  functions: HashMap<
    String,
    Box<dyn Fn(Vec<Expr>, &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> + Send + Sync>,
  >,
  pub current_token: LabeledToken,
}

impl<'a> Parser<'a> {
  pub fn new(lexer: Lexer<'a>) -> Self {
    let mut parser = Parser {
      lexer,
      functions: load_libs(),
      current_token: LabeledToken::default(),
    };

    parser.next_token(); // Initialize the first token

    parser
  }

  pub fn parse(&mut self) -> Result<Option<Expr>, ParseError> {
    let expr = match &self.current_token.token {
      Token::EOF => Ok(None),
      Token::Let => self.parse_let(),
      Token::Const => self.parse_const(),
      Token::If => self.parse_statement(),
      Token::For => self.parse_for(),
      Token::Function => self.parse_function(),
      Token::Import => self.parse_import(),
      Token::Return => self.parse_return(),
      Token::Symbol(_) => self.parse_expression().map_err(|e| e),
      _ => Err(ParseError::UnexpectedToken(
        self.current_token.clone().token,
      )),
    };

    self.try_eat(Token::Semicolon)?;

    expr
  }

  fn eat(&mut self, token: Token) -> Result<(), ParseError> {
    if &self.current_token.token == &token {
      self.next_token(); // Move to the next token
      Ok(())
    } else {
      Err(ParseError::ExpectedToken(
        self.current_token.line_number,
        self.current_token.token.clone(),
        token,
      ))
    }
  }

  pub fn try_eat(&mut self, token: Token) -> Result<(), ParseError> {
    if &self.current_token.token == &token {
      self.next_token(); // Consume semicolon
    }

    Ok(())
  }

  fn next_token(&mut self) {
    self.current_token = self.lexer.next_token();
  }
}
