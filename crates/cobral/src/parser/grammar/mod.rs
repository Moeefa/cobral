use super::{error::ParserError, Parser};
use crate::shared::ast::{Expression, Statement};

pub mod expressions;
pub mod statements;

impl Parser {
  pub fn parse_expression(&mut self) -> Result<Expression, ParserError> {
    expressions::parse_expression(self)
  }

  pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
    statements::parse_statement(self)
  }
}
