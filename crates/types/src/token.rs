use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  // Keywords
  Let,
  Const,
  If,
  Else,
  For,
  Function,
  Return,

  // Symbols
  Equals,

  // Punctuation
  Semicolon,

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
  Symbol(String),
  Integer(i64),
  Float(f64),
  String(String),
  List(Vec<LabeledToken>),

  // Booleans
  True,
  False,

  // Operators
  GreaterThan,
  GreaterThanEqual,
  LessThan,
  LessThanEqual,
  EqualEqual,
  NotEqual,
  Not,
  And,
  Or,

  // Binary
  Plus,
  Minus,
  Times,
  Divide,
  Rem,

  // Unary
  Unary,

  // End of file
  EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabeledToken {
  pub token: Token,
  pub line_number: usize,
}

impl Default for LabeledToken {
  fn default() -> Self {
    LabeledToken {
      token: Token::EOF,
      line_number: 0,
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
      Token::For => write!(f, "para"),
      Token::Function => write!(f, "funcao"),
      Token::Return => write!(f, "retorne"),
      Token::Equals => write!(f, "="),
      Token::Semicolon => write!(f, ";"),
      Token::ParenL => write!(f, "("),
      Token::ParenR => write!(f, ")"),
      Token::BracketL => write!(f, "["),
      Token::BracketR => write!(f, "]"),
      Token::BraceL => write!(f, "{{"),
      Token::BraceR => write!(f, "}}"),
      Token::Comma => write!(f, ","),
      Token::Symbol(s) => write!(f, "{}", s),
      Token::Integer(n) => write!(f, "{}", n),
      Token::Float(n) => write!(f, "{}", n),
      Token::String(s) => write!(f, "{}", s),
      Token::List(l) => write!(f, "{:?}", l),
      Token::True => write!(f, "verdadeiro"),
      Token::False => write!(f, "falso"),
      Token::GreaterThan => write!(f, ">"),
      Token::GreaterThanEqual => write!(f, ">="),
      Token::LessThan => write!(f, "<"),
      Token::LessThanEqual => write!(f, "<="),
      Token::EqualEqual => write!(f, "=="),
      Token::NotEqual => write!(f, "!="),
      Token::Not => write!(f, "nao"),
      Token::And => write!(f, "e"),
      Token::Or => write!(f, "ou"),
      Token::Plus => write!(f, "+"),
      Token::Minus => write!(f, "-"),
      Token::Times => write!(f, "*"),
      Token::Divide => write!(f, "/"),
      Token::Rem => write!(f, "%"),
      Token::Unary => write!(f, "unario"),
      Token::EOF => write!(f, "EOF"),
    }
  }
}
