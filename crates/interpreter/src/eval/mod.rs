pub mod assignment;
pub mod binary;
pub mod comparison;
pub mod r#const;
pub mod function;
pub mod import;
pub mod index;
pub mod r#let;
pub mod list;
pub mod logical;
pub mod r#loop;
pub mod postfix;
pub mod prefix;
pub mod statement;
pub mod switch;
pub mod unary;
pub mod r#while;

use ::enums::{Data, Expr, LabeledExpr};

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_block(&self, block: &Vec<Expr>) -> Result<Data, InterpreterError> {
    for expr in block {
      self.eval(LabeledExpr {
        expr: expr.clone(),
        line_number: 0, // Adjust line number tracking
      })?;
    }

    Ok(Data::None)
  }
}
