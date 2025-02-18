use crate::lexer::token::Token;

use super::Location;

#[derive(Debug, Clone)]
pub enum Expression {
  // Arithmetic Operations
  Arithmetic {
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
    location: Location,
  },

  // Unary Operations
  Unary {
    operator: Token,
    expr: Box<Expression>,
    location: Location,
  },

  // Comparison Operations
  Comparison {
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
    location: Location,
  },

  // Logical Operations
  Logical {
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
    location: Location,
  },
  // And(Box<Expression>, Box<Expression>),
  // Or(Box<Expression>, Box<Expression>),

  // Data Types and Literals
  Integer(i64, Location),
  Float(f64, Location),
  String(String, Location),
  Boolean(bool, Location),
  Nil(Location),

  // Functions
  Call {
    callee: Box<Expression>,
    arguments: Vec<Expression>,
    location: Location,
  },

  // Symbol and operator-related
  Identifier(String, Location),

  // List Operations
  List(Vec<Expression>, Location),
  Index {
    name: String,
    index: Box<Expression>,
    location: Location,
  },

  // Increment and Decrement
  PostfixIncrement(Box<Expression>, Location),
  PostfixDecrement(Box<Expression>, Location),
  PrefixIncrement(Box<Expression>, Location),
  PrefixDecrement(Box<Expression>, Location),
}

impl Expression {
  pub fn is_literal(&self) -> bool {
    match self {
      Expression::Integer(_, _) => true,
      Expression::Float(_, _) => true,
      Expression::String(_, _) => true,
      Expression::Boolean(_, _) => true,
      Expression::Nil(_) => true,
      _ => false,
    }
  }

  pub fn location(&self) -> Location {
    match self {
      Expression::Logical { location, .. } => location.clone(),
      Expression::Comparison { location, .. } => location.clone(),
      Expression::Arithmetic { location, .. } => location.clone(),
      Expression::Unary { location, .. } => location.clone(),
      Expression::Integer(_, location) => location.clone(),
      Expression::Float(_, location) => location.clone(),
      Expression::String(_, location) => location.clone(),
      Expression::Boolean(_, location) => location.clone(),
      Expression::Nil(location) => location.clone(),
      Expression::Call { location, .. } => location.clone(),
      Expression::Identifier(_, location) => location.clone(),
      Expression::List(_, location) => location.clone(),
      Expression::Index { location, .. } => location.clone(),
      Expression::PostfixIncrement(_, location) => location.clone(),
      Expression::PostfixDecrement(_, location) => location.clone(),
      Expression::PrefixIncrement(_, location) => location.clone(),
      Expression::PrefixDecrement(_, location) => location.clone(),
    }
  }
}

// Implement necessary traits
impl std::fmt::Display for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Expression::Arithmetic {
        left,
        operator,
        right,
        location: _,
      } => {
        write!(f, "({} {} {})", left, operator, right)
      }
      Expression::Integer(n, _) => write!(f, "{}", n),
      Expression::Float(n, _) => write!(f, "{}", n),
      // Add other formatting cases
      _ => write!(f, "{:?}", self),
    }
  }
}
