mod arguments;
mod consts;
mod expressions;
mod lists;
mod statements;
mod vars;

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
    match &self.current_token.token {
      Token::EOF => Ok(None),
      Token::Let => self.parse_vars(),
      Token::Const => self.parse_const(),
      Token::If => self.parse_statement(),
      Token::Symbol(_) => self.parse_expression().map_err(|e| e),
      _ => Err(ParseError::UnexpectedToken(
        self.current_token.clone().token,
      )),
    }
  }

  pub fn skip_semicolon(&mut self) {
    if matches!(self.current_token.token, Token::Semicolon) {
      self.next_token(); // Consume semicolon
    }
  }

  fn eat(&mut self, token: Token) {
    if &self.current_token.token == &token {
      self.next_token(); // Move to the next token
    } else {
      panic!("Unexpected token: {:?}", self.current_token.token);
    }

    self.skip_semicolon();
  }

  fn next_token(&mut self) {
    self.current_token = self.lexer.next_token();
  }
}
