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
mod switch;
mod r#while;

use ::enums::{Expr, LabeledExpr, LabeledToken, Token};
use context::Context;
use enums::errors::ParserError;
use lexer::Lexer;
use std::sync::Arc;

// Custom type for parser results to reduce repetition
type ParseResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub struct Parser<'a> {
  lexer: Lexer<'a>,
  context: Arc<Context>, // Use Arc for thread-safe sharing
  current_token: LabeledToken,
  peek_token: Option<LabeledToken>, // Add token lookahead
}

impl<'a> Parser<'a> {
  pub fn new(lexer: Lexer<'a>) -> ParseResult<Vec<LabeledExpr>> {
    let mut parser = Parser {
      lexer,
      context: Arc::new(Context::new()),
      current_token: LabeledToken::default(),
      peek_token: None,
    };

    parser.next_token(); // Initialize first token
    parser.peek_token(); // Initialize peek token

    parser.run()
  }

  // Add peek functionality
  fn peek_token(&mut self) {
    if self.peek_token.is_none() {
      self.peek_token = Some(self.lexer.next_token());
    }
  }

  fn run(&mut self) -> ParseResult<Vec<LabeledExpr>> {
    let mut exprs = Vec::with_capacity(32); // Preallocate with reasonable capacity

    while self.current_token.token != Token::EOF {
      if let Some(expr) = self.parse()? {
        exprs.push(LabeledExpr {
          expr,
          line_number: self.current_token.line_number,
        });
      }
    }

    Ok(exprs)
  }

  // Optimize type checking with a lookup table
  fn expr_type(&self, expr: &Expr) -> &'static str {
    use std::collections::HashMap;
    use std::sync::OnceLock;

    static BUILTIN_FUNCTIONS: OnceLock<HashMap<&str, &str>> = OnceLock::new();
    let builtins = BUILTIN_FUNCTIONS.get_or_init(|| {
      let mut m = HashMap::new();
      m.insert("escrever", "cadeia");
      m.insert("ler", "cadeia");
      m.insert("raiz", "real");
      m.insert("potencia", "real");
      m.insert("real", "real");
      m.insert("int", "real");
      m
    });

    match expr {
      Expr::Symbol(s) => {
        let variables = self.context.variables.lock().ok();
        let constants = self.context.constants.lock().ok();

        if let (Some(vars), Some(consts)) = (variables, constants) {
          if let Some(data) = vars.get(s).cloned().or_else(|| consts.get(s).cloned()) {
            if let Some(expr) = data.as_ref() {
              return self.expr_type(expr);
            }
          }
        }
        "desconhecido"
      }
      Expr::Integer(_) => "inteiro",
      Expr::List(_) => "lista",
      Expr::Float(_) => "real",
      Expr::String(_) => "cadeia",
      Expr::Boolean(_) => "lógico",
      Expr::FunctionCall(name, _) => builtins
        .get(name.as_str())
        .copied()
        .unwrap_or("desconhecido"),
      _ => "desconhecido",
    }
  }

  fn parse(&mut self) -> ParseResult<Option<Expr>> {
    let expr = match &self.current_token.token {
      Token::EOF => return Ok(None),
      Token::Let => self.parse_let(),
      Token::Const => self.parse_const(),
      Token::If => self.parse_statement(),
      Token::Switch => self.parse_switch_statement(),
      Token::For => self.parse_for(),
      Token::While => self.parse_while(),
      Token::Function => self.parse_function(),
      Token::Import => self.parse_import(),
      Token::Return => self.parse_return(),
      Token::Symbol(_) => self.parse_expression(),
      _ => Err(ParserError::UnexpectedToken(self.current_token.clone())),
    }?;

    self.try_eat(Token::Semicolon)?;
    Ok(expr)
  }

  fn parse_block(&mut self) -> ParseResult<Vec<Expr>> {
    self.eat(Token::BraceL)?;

    let mut exprs = Vec::with_capacity(8); // Preallocate with reasonable capacity

    while !matches!(self.current_token.token, Token::BraceR | Token::EOF) {
      if let Some(expr) = self.parse()? {
        exprs.push(expr);
      }
    }

    if self.current_token.token == Token::BraceR {
      self.eat(Token::BraceR)?;
    } else {
      return Err(ParserError::ExpectedToken(
        self.current_token.clone(),
        Token::BraceR,
      ));
    }

    Ok(exprs)
  }

  // Optimize token consumption with peek
  fn eat(&mut self, expected: Token) -> ParseResult<()> {
    if self.current_token.token == expected {
      self.next_token();
      Ok(())
    } else {
      Err(ParserError::ExpectedToken(
        self.current_token.clone(),
        expected,
      ))
    }
  }

  fn try_eat(&mut self, token: Token) -> ParseResult<()> {
    if self.current_token.token == token {
      self.next_token();
    }
    Ok(())
  }

  fn next_token(&mut self) {
    if let Some(next) = self.peek_token.take() {
      self.current_token = next;
      self.peek_token();
    } else {
      self.current_token = self.lexer.next_token();
    }
  }
}
