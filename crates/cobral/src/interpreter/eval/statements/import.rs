use std::{fs, path::Path, sync::Arc};

use crate::{
  interpreter::{builtin, error::InterpreterError, value::Value, Interpreter, LibFn},
  lexer::Lexer,
  parser::Parser,
};

impl Interpreter {
  pub fn eval_import_stmt(&mut self, file: String) -> Result<Value, InterpreterError> {
    if let Some(lib) = builtin::load(&file) {
      // Add functions from the library to the interpreter's library map
      for (name, func) in lib {
        self
          .environment
          .libs
          .write()
          .insert(name.to_string(), Arc::new(func) as LibFn);
      }

      return Ok(Value::None);
    }

    let path = Path::new(&file);

    if !path.exists() {
      return Err(InterpreterError::FileNotFound(file));
    }

    let code = fs::read_to_string(path)
      .map_err(|e| InterpreterError::FileReadError(file.clone(), e.to_string()))?;

    // Evaluate the imported file
    let tokens = match Lexer::new(&code) {
      Ok(tokens) => tokens,
      Err(e) => {
        return Err(InterpreterError::FileReadError(file.clone(), e.to_string()));
      }
    };

    let stmts = match Parser::new(tokens) {
      Ok(stmts) => stmts,
      Err(e) => {
        return Err(InterpreterError::FileReadError(file.clone(), e.to_string()));
      }
    };

    for stmt in stmts {
      match self.eval_stmt(&stmt) {
        Ok(_) => {}
        Err(e) => return Err(InterpreterError::FileReadError(file.clone(), e.to_string())),
      }
    }

    Ok(Value::None)
  }
}
