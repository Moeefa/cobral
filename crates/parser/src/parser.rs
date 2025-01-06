mod argument;
mod r#const;
mod context;
mod enums;
mod expressions;
mod function;
mod import;
mod r#let;
mod list;
mod r#loop;
mod r#return;
mod statement;

use ::enums::{Expr, LabeledToken, Token};
use context::Context;
use enums::errors::ParserError;
use lexer::Lexer;

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
      Expr::Symbol(s) => {
        let variables = self.context.variables.lock().unwrap();
        let constants = self.context.constants.lock().unwrap();
        if let Some(data) = variables
          .get(s)
          .cloned()
          .or_else(|| constants.get(s).cloned())
        {
          if let Some(expr) = data.as_ref() {
            self.expr_type(expr)
          } else {
            Some("desconhecido")
          }
        } else {
          Some("indefinido")
        }
      }
      Expr::Integer(_) => Some("inteiro"),
      Expr::List(_) => Some("lista"),
      Expr::Float(_) => Some("real"),
      Expr::String(_) => Some("cadeia"),
      Expr::Boolean(_) => Some("lógico"),
      Expr::FunctionCall(name, _) => match name.as_str() {
        "escrever" | "ler" => Some("cadeia"),
        "raiz" | "potencia" | "real" => Some("real"),
        "int" => Some("real"),
        _ => Some("desconhecido"), // Unknown or user-defined function TODO: Add user-defined function support
      },
      _ => Some("desconhecido"), // Unknown or unsupported expression
    }
  }

  pub fn parse(&mut self) -> Result<Option<Expr>, ParserError> {
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
      _ => Err(ParserError::UnexpectedToken(
        self.current_token.line_number,
        self.current_token.clone().token,
      )),
    };

    self.try_eat(Token::Semicolon)?;

    expr
  }

  pub fn parse_block(&mut self) -> Result<Vec<Expr>, ParserError> {
    self.eat(Token::BraceL)?; // Consume `{`

    let mut exprs = Vec::new();

    while self.current_token.token != Token::BraceR && self.current_token.token != Token::EOF {
      if let Some(expr) = self.parse()? {
        exprs.push(expr);
      } else {
        return Err(ParserError::UnexpectedToken(
          self.current_token.line_number,
          self.current_token.clone().token,
        ));
      }
    }

    if self.current_token.token == Token::BraceR {
      self.eat(Token::BraceR)?; // Consume `}`
    } else {
      return Err(ParserError::UnexpectedToken(
        self.current_token.line_number,
        self.current_token.clone().token,
      ));
    }

    Ok(exprs)
  }

  fn eat(&mut self, token: Token) -> Result<(), ParserError> {
    if &self.current_token.token == &token {
      self.next_token(); // Move to the next token
      Ok(())
    } else {
      Err(ParserError::ExpectedToken(
        self.current_token.line_number,
        self.current_token.token.clone(),
        token,
      ))
    }
  }

  pub fn try_eat(&mut self, token: Token) -> Result<(), ParserError> {
    if &self.current_token.token == &token {
      self.next_token(); // Consume semicolon
    }

    Ok(())
  }

  fn next_token(&mut self) {
    self.current_token = self.lexer.next_token();
  }
}
