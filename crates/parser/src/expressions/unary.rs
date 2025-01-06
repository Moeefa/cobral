use ::enums::{Expr, Token};

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_unary_expression(&mut self) -> Result<Option<Expr>, ParserError> {
    match &self.current_token.token {
      Token::Minus => {
        self.eat(Token::Minus)?; // Consume the `-` token
        let operand = self
          .parse_primary_expression()?
          .ok_or(ParserError::InvalidExpression(
            self.current_token.line_number,
            "Operando ausente após unário '-'".to_string(),
          ))?;
        Ok(Some(Expr::UnaryMinus(Box::new(operand))))
      }
      Token::Increment => {
        self.eat(Token::Increment)?;
        let expr = self.parse_primary_expression()?;
        Ok(Some(Expr::PrefixIncrement(Box::new(expr.unwrap()))))
      }
      Token::Decrement => {
        self.eat(Token::Decrement)?;
        let expr = self.parse_primary_expression()?;
        Ok(Some(Expr::PrefixDecrement(Box::new(expr.unwrap()))))
      }
      Token::Not => {
        self.eat(Token::Not)?; // Move past 'not'
        let right_expr = self.parse_expression().unwrap(); // Parse the expression after 'not'

        if let Some(right_expr) = right_expr {
          match right_expr {
            Expr::Boolean(_) => {
              return Ok(Some(Expr::UnaryNot(Box::new(right_expr))));
            }
            _ => {
              return Err(ParserError::InvalidExpression(
                self.current_token.line_number,
                "Operador 'não' deve ser aplicado a um valor booleano".to_string(),
              ));
            }
          };
        };

        return Err(ParserError::InvalidExpression(
          self.current_token.line_number,
          "Operador 'não' deve ser aplicado a um valor booleano".to_string(),
        ));
      }
      _ => self.parse_primary_expression(),
    }
  }
}
