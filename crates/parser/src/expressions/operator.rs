use ::enums::Expr;

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_expression_operator(&mut self, lhs: Expr) -> Result<Option<Expr>, ParserError> {
    // Handle comparison operators
    let op = self.current_token.token.clone();
    self.eat(op.clone())?; // Consume the operator (e.g., '>', '<', '==', etc.)

    // Parse the right-hand side (RHS) expression
    let rhs = self.parse_expression()?.unwrap(); // Recursively parse the RHS

    Ok(Some(Expr::Comparison(Box::new(lhs), op, Box::new(rhs))))
  }
}
