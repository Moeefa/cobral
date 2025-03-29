use std::{
  collections::{BTreeMap, HashMap},
  sync::Arc,
};

use super::{
  builtin::io::{error, read, write},
  value::Value,
  LibFn,
};
use crate::shared::ast::{Expression, Statement};
use parking_lot::RwLock;

#[derive(Debug, Clone, PartialEq)]
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
  pub input_result: Arc<RwLock<Option<Value>>>,
  pub pending_function_call: Arc<RwLock<Option<(String, Vec<Expression>)>>>,
  fn_scope_depth: usize,
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

  pub fn delete(&self, name: &str) -> Result<(), EnvironmentError> {
    let mut symbols = self.symbols.write();
    if symbols.remove(name).is_some() {
      Ok(())
    } else {
      Err(EnvironmentError::SymbolNotFound)
    }
  }

  // Changed from define_input_result to set_input_result
  pub fn set_input_result(&self, value: Value) {
    let mut input = self.input_result.write();
    *input = Some(value);
  }

  // Change this method to not require &mut self
  pub fn take_input_result(&self) -> Option<Value> {
    let mut input = self.input_result.write();
    input.take()
  }

  pub fn set_pending_function_call(&self, function_name: String, args: Vec<Expression>) {
    let mut pending = self.pending_function_call.write();
    *pending = Some((function_name, args));
  }

  pub fn take_pending_function_call(&self) -> Option<(String, Vec<Expression>)> {
    let mut pending = self.pending_function_call.write();
    pending.take()
  }
  
  // Track if we're currently in a function scope
  pub fn enter_function_scope(&mut self) {
    self.fn_scope_depth += 1;
  }
  
  pub fn exit_function_scope(&mut self) {
    if self.fn_scope_depth > 0 {
      self.fn_scope_depth -= 1;
    }
  }
  
  pub fn is_in_function_scope(&self) -> bool {
    self.fn_scope_depth > 0
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
      input_result: Arc::new(RwLock::new(None)),
      pending_function_call: Arc::new(RwLock::new(None)),
      fn_scope_depth: 0,
    }
  }
}
