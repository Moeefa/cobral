mod argument;
mod r#const;
mod enums;
mod expressions;
mod function;
mod import;
mod r#let;
mod list;
mod r#loop;
mod r#return;
mod statement;
mod switch;
mod r#while;

use std::{collections::HashMap, sync::Arc};

use ::enums::{Expr, LabeledExpr, LabeledToken, Token};
use enums::errors::ParserError;
use lexer::Lexer;
use parking_lot::RwLock;

#[derive(Debug)]
pub struct Environment {
  pub constants: Arc<RwLock<HashMap<String, Option<Expr>>>>,
  pub variables: Arc<RwLock<HashMap<String, Option<Expr>>>>,
  pub functions: Arc<RwLock<HashMap<String, Option<Vec<String>>>>>,
  pub libs: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl Environment {
  pub fn new() -> Self {
    Environment {
      constants: Arc::new(RwLock::new(HashMap::new())),
      variables: Arc::new(RwLock::new(HashMap::new())),
      functions: Arc::new(RwLock::new(HashMap::new())),
      libs: Arc::new(RwLock::new(HashMap::from([(
        "io".to_string(),
        vec![
          "escrever".to_string(),
          "erro".to_string(),
          "ler".to_string(),
        ],
      )]))),
    }
  }
}

impl Default for Environment {
  fn default() -> Self {
    Self::new()
  }
}

#[allow(dead_code)]
pub struct Parser<'a> {
  lexer: Lexer<'a>,
  env: Environment,
  pub current_token: LabeledToken,
}

impl<'a> Parser<'a> {
  pub fn new(lexer: Lexer<'a>) -> Result<Vec<LabeledExpr>, ParserError> {
    let mut parser = Parser {
      lexer,
      env: Environment::default(),
      current_token: LabeledToken::default(),
    };

    parser.next_token(); // Initialize the first token

    Ok(parser.run()?)
  }

  fn run(&mut self) -> Result<Vec<LabeledExpr>, ParserError> {
    let mut exprs = Vec::new();

    while self.current_token.token != Token::EOF {
      if let Some(expr) = self.parse()? {
        exprs.push(LabeledExpr {
          expr,
          line_number: self.current_token.line_number,
        });
      } else {
        return Err(ParserError::UnexpectedToken(self.current_token.clone()));
      }
    }

    Ok(exprs)
  }

  fn expr_type(&self, expr: &Expr) -> &str {
    match expr {
      Expr::Symbol(s) => {
        let variables = self.env.variables.read();
        let constants = self.env.constants.read();
        if let Some(data) = variables
          .get(s)
          .cloned()
          .or_else(|| constants.get(s).cloned())
        {
          if let Some(expr) = data.as_ref() {
            self.expr_type(expr)
          } else {
            "desconhecido"
          }
        } else {
          "indefinido"
        }
      }
      Expr::Integer(_) => "inteiro",
      Expr::List(_) => "lista",
      Expr::Float(_) => "real",
      Expr::String(_) => "cadeia",
      Expr::Boolean(_) => "lógico",
      Expr::FunctionCall(name, _) => match name.as_str() {
        "escrever" | "ler" => "cadeia",
        "raiz" | "potencia" => "real",
        "real" => "inteiro",
        "int" => "real",
        _ => "desconhecido", // Unknown or user-defined function TODO: Add user-defined function support
      },
      _ => "desconhecido", // Unknown or unsupported expression
    }
  }

  fn parse(&mut self) -> Result<Option<Expr>, ParserError> {
    let expr = match &self.current_token.token {
      Token::EOF => Ok(None),
      Token::Let => self.parse_let(),
      Token::Const => self.parse_const(),
      Token::If => self.parse_statement(),
      Token::Switch => self.parse_switch_statement(),
      Token::For => self.parse_for(),
      Token::While => self.parse_while(),
      Token::Function => self.parse_function(),
      Token::Import => self.parse_import(),
      Token::Return => self.parse_return(),
      Token::Symbol(_) => self.parse_expression().map_err(|e| e),
      _ => Err(ParserError::UnexpectedToken(self.current_token.clone())),
    };

    self.try_eat(Token::Semicolon)?;

    expr
  }

  fn parse_block(&mut self) -> Result<Vec<Expr>, ParserError> {
    self.eat(Token::BraceL)?; // Consume `{`

    let mut exprs = Vec::new();

    while self.current_token.token != Token::BraceR && self.current_token.token != Token::EOF {
      if let Some(expr) = self.parse()? {
        exprs.push(expr);
      } else {
        return Err(ParserError::UnexpectedToken(self.current_token.clone()));
      }
    }

    if self.current_token.token == Token::BraceR {
      self.eat(Token::BraceR)?; // Consume `}`
    } else {
      return Err(ParserError::UnexpectedToken(self.current_token.clone()));
    }

    Ok(exprs)
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
    self.current_token = self.lexer.next_token();
  }
}
