mod argument;
mod r#const;
mod context;
mod expressions;
mod function;
mod import;
mod r#let;
mod list;
mod r#loop;
mod r#return;
mod statement;

use context::Context;
use lexer::Lexer;
use types::{Expr, LabeledToken, ParseError, Token};

#[allow(dead_code)]
pub struct Parser<'a> {
  lexer: Lexer<'a>,
  context: Context,
  pub current_token: LabeledToken,
}

impl<'a> Parser<'a> {
  pub fn new(lexer: Lexer<'a>) -> Self {
    let mut parser = Parser {
      lexer,
      context: Context::new(),
      current_token: LabeledToken::default(),
    };

    parser.next_token(); // Initialize the first token

    parser
  }

  pub fn expr_type(&self, expr: &Expr) -> Option<&'static str> {
    match expr {
      Expr::Symbol(s) => self
        .expr_type(self.context.variables.lock().unwrap().get(s)?)
        .or_else(|| self.expr_type(self.context.constants.lock().unwrap().get(s)?)),
      Expr::Integer(_) => Some("inteiro"),
      Expr::List(_) => Some("lista"),
      Expr::Float(_) => Some("real"),
      Expr::String(_) => Some("cadeia"),
      Expr::Boolean(_) => Some("lógico"),
      Expr::FunctionCall(name, _) => match name.as_str() {
        "escrever" | "ler" => Some("cadeia"),
        "raiz" | "potencia" | "real" => Some("real"),
        "int" => Some("real"),
        _ => None, // Unknown or user-defined function TODO: Add user-defined function support
      },
      _ => None, // Unknown or unsupported expression
    }
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
        self.current_token.line_number,
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
