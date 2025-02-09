use std::{
  collections::{BTreeMap, HashMap},
  sync::Arc,
};

use super::{
  builtin::io::{error, read, write},
  value::Value,
  LibFn,
};
use crate::shared::ast::Statement;
use parking_lot::RwLock;

#[derive(Debug, Clone)]
pub enum EnvironmentError {
  SymbolAlreadyDefined,
  ConstantReassignment,
  SymbolNotFound,
}

impl std::fmt::Display for EnvironmentError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      EnvironmentError::SymbolAlreadyDefined => write!(f, "Symbol already defined"),
      EnvironmentError::ConstantReassignment => write!(f, "Cannot reassign to a constant"),
      EnvironmentError::SymbolNotFound => write!(f, "Symbol not found"),
    }
  }
}

impl std::error::Error for EnvironmentError {}

#[derive(Clone, Debug)]
pub enum Symbol {
  Variable(Value),
  Constant(Value),
}

impl Symbol {
  pub fn get_value(&self) -> &Value {
    match self {
      Symbol::Variable(val) | Symbol::Constant(val) => val,
    }
  }

  pub fn set_value(&mut self, new_value: Value) -> Result<(), EnvironmentError> {
    match self {
      Symbol::Variable(val) => {
        *val = new_value;
        Ok(())
      }
      Symbol::Constant(_) => Err(EnvironmentError::ConstantReassignment),
    }
  }
}

#[derive(Clone)]
pub struct Environment {
  pub symbols: Arc<RwLock<BTreeMap<String, Symbol>>>,
  pub functions: Arc<RwLock<HashMap<String, (Vec<String>, Vec<Statement>)>>>,
  pub libs: Arc<RwLock<HashMap<String, LibFn>>>,
}

impl Environment {
  pub fn define_variable(&self, name: String, value: Value) -> Result<(), EnvironmentError> {
    let mut symbols = self.symbols.write();
    match symbols.get(&name) {
      Some(Symbol::Constant(_)) => Err(EnvironmentError::SymbolAlreadyDefined),
      Some(Symbol::Variable(_)) => {
        symbols.insert(name, Symbol::Variable(value));
        Ok(())
      }
      None => {
        symbols.insert(name, Symbol::Variable(value));
        Ok(())
      }
    }
  }

  pub fn define_constant(&self, name: String, value: Value) -> Result<(), EnvironmentError> {
    let mut symbols = self.symbols.write();
    if symbols.contains_key(&name) {
      return Err(EnvironmentError::SymbolAlreadyDefined);
    }
    symbols.insert(name, Symbol::Constant(value));
    Ok(())
  }

  pub fn get_symbol(&self, name: &str) -> Option<Arc<RwLock<Symbol>>> {
    self
      .symbols
      .read()
      .get(name)
      .map(|symbol| Arc::new(RwLock::new(symbol.clone())))
  }

  pub fn set_symbol_value(&self, name: &str, new_value: Value) -> Result<(), EnvironmentError> {
    let mut symbols = self.symbols.write();
    if let Some(symbol) = symbols.get_mut(name) {
      symbol.set_value(new_value)?;
      Ok(())
    } else {
      Err(EnvironmentError::SymbolNotFound)
    }
  }

  pub fn get_lib(&self, name: &str) -> Option<LibFn> {
    self.libs.read().get(name).cloned()
  }

  pub fn get_function(&self, name: &str) -> Option<(Vec<String>, Vec<Statement>)> {
    self.functions.read().get(name).cloned()
  }
}

impl Default for Environment {
  fn default() -> Self {
    let default_libs = HashMap::from([
      ("escrever".to_string(), Arc::new(write) as LibFn),
      ("erro".to_string(), Arc::new(error) as LibFn),
      ("ler".to_string(), Arc::new(read) as LibFn),
    ]);

    Environment {
      symbols: Arc::new(RwLock::new(BTreeMap::new())),
      functions: Arc::new(RwLock::new(HashMap::new())),
      libs: Arc::new(RwLock::new(default_libs)),
    }
  }
}
