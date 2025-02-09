use std::fmt;

use crate::shared::ast::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  // Keywords
  Let,
  Const,
  If,
  Else,
  Switch,
  Case,
  Default,
  For,
  While,
  Function,
  Return,
  Break,
  Import,

  // Symbols
  Equal,

  // Punctuation
  Semicolon,
  Colon,

  // Delimiters
  ParenL,
  ParenR,
  BracketL,
  BracketR,
  BraceL,
  BraceR,

  // Other
  Comma,

  // Values
  Identifier(String),
  Integer(i64),
  Float(f64),
  String(String),

  // Booleans
  True,
  False,

  // Operators
  Greater,
  GreaterEquals,
  Less,
  LessEquals,
  Equals,
  NotEquals,
  Not,
  And,
  Or,

  // Arithmetic
  Plus,
  Minus,
  Asterisk,
  Slash,
  Rem,
  Increment, // For `++`
  Decrement, // For `--`

  // End of file
  EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabeledToken {
  pub token: Token,
  pub location: Location,
}

impl Default for LabeledToken {
  fn default() -> Self {
    LabeledToken {
      token: Token::EOF,
      location: Location { line: 0, column: 0 },
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Token::Let => write!(f, "declare"),
      Token::Const => write!(f, "declare constante"),
      Token::If => write!(f, "se"),
      Token::Else => write!(f, "senao"),
      Token::Switch => write!(f, "escolha"),
      Token::Case => write!(f, "caso"),
      Token::Default => write!(f, "padrao"),
      Token::For => write!(f, "para"),
      Token::While => write!(f, "enquanto"),
      Token::Function => write!(f, "funcao"),
      Token::Return => write!(f, "retorne"),
      Token::Break => write!(f, "pare"),
      Token::Import => write!(f, "importar"),
      Token::Equal => write!(f, "="),
      Token::Semicolon => write!(f, ";"),
      Token::Colon => write!(f, ":"),
      Token::ParenL => write!(f, "("),
      Token::ParenR => write!(f, ")"),
      Token::BracketL => write!(f, "["),
      Token::BracketR => write!(f, "]"),
      Token::BraceL => write!(f, "{{"),
      Token::BraceR => write!(f, "}}"),
      Token::Comma => write!(f, ","),
      Token::Identifier(s) => write!(f, "{}", s),
      Token::Integer(n) => write!(f, "{}", n),
      Token::Float(n) => write!(f, "{}", n),
      Token::String(s) => write!(f, "{}", s),
      Token::True => write!(f, "verdadeiro"),
      Token::False => write!(f, "falso"),
      Token::Greater => write!(f, ">"),
      Token::GreaterEquals => write!(f, ">="),
      Token::Less => write!(f, "<"),
      Token::LessEquals => write!(f, "<="),
      Token::Equals => write!(f, "=="),
      Token::NotEquals => write!(f, "!="),
      Token::Not => write!(f, "nao"),
      Token::And => write!(f, "e"),
      Token::Or => write!(f, "ou"),
      Token::Plus => write!(f, "+"),
      Token::Minus => write!(f, "-"),
      Token::Asterisk => write!(f, "*"),
      Token::Slash => write!(f, "/"),
      Token::Rem => write!(f, "%"),
      Token::Increment => write!(f, "++"),
      Token::Decrement => write!(f, "--"),
      Token::EOF => write!(f, "EOF"),
    }
  }
}
