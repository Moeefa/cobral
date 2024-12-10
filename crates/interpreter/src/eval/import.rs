use std::{fs, path::Path};

use lexer::Lexer;
use parser::Parser;
use types::{Data, InterpreterError, LabeledExpr};

use crate::Interpreter;

impl Interpreter {
  pub fn eval_import(&self, file: String) -> Result<Data, InterpreterError> {
    let path = Path::new(&file);

    if !path.exists() {
      return Err(InterpreterError::FileNotFound(file));
    }

    let code = fs::read_to_string(path)
      .map_err(|e| InterpreterError::FileReadError(file.clone(), e.to_string()))?;

    // Evaluate the imported file
    let lexer = Lexer::new(&code);
    let mut parser = Parser::new(lexer);

    while let Some(expr) = parser.parse().unwrap() {
      self.eval(LabeledExpr {
        expr,
        line_number: 0,
      })?;
    }

    Ok(Data::None)
  }
}
