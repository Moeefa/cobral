pub mod assignment;
pub mod binary;
pub mod comparison;
pub mod r#const;
pub mod function;
pub mod import;
pub mod r#let;
pub mod list;
pub mod logical;
pub mod r#loop;
pub mod postfix;
pub mod prefix;
pub mod statement;
pub mod unary;

use types::{Data, Expr, InterpreterError, LabeledExpr};

use crate::Interpreter;

impl Interpreter {
  pub fn eval_block(&self, block: Vec<Expr>) -> Result<Data, InterpreterError> {
    for expr in block {
      self.eval(LabeledExpr {
        expr,
        line_number: 0, // Adjust line number tracking
      })?;
    }
    Ok(Data::None)
  }
}
