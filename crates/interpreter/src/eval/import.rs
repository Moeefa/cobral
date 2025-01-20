use std::{fs, path::Path, sync::Arc};

use ::enums::Data;
use lexer::Lexer;
use parser::Parser;

use crate::{enums::errors::InterpreterError, Interpreter, LibFn};

impl Interpreter {
  pub fn eval_import(&self, file: String) -> Result<Data, InterpreterError> {
    if let Some(lib) = libs::load(&file) {
      // Add functions from the library to the interpreter's library map
      for (name, func) in lib {
        self
          .env
          .libs
          .write()
          .insert(name.to_string(), Arc::new(func) as LibFn);
      }

      return Ok(Data::None);
    }

    let path = Path::new(&file);

    if !path.exists() {
      return Err(InterpreterError::FileNotFound(file));
    }

    let code = fs::read_to_string(path)
      .map_err(|e| InterpreterError::FileReadError(file.clone(), e.to_string()))?;

    // Evaluate the imported file
    let lexer = Lexer::new(&code);
    let parser = Parser::new(lexer);
    let exprs = parser.unwrap_or_else(|e| {
      eprintln!("{}", e);
      Vec::new()
    });

    for expr in exprs {
      self.eval(expr)?;
    }

    Ok(Data::None)
  }
}
