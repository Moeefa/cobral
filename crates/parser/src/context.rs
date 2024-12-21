use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use types::Expr;

pub struct Context {
  pub constants: Arc<Mutex<HashMap<String, Expr>>>,
  pub variables: Arc<Mutex<HashMap<String, Expr>>>,
  pub functions: Arc<Mutex<HashMap<String, (Vec<String>, Vec<Expr>)>>>,
}

impl Context {
  pub fn new() -> Self {
    Context {
      constants: Arc::new(Mutex::new(HashMap::new())),
      variables: Arc::new(Mutex::new(HashMap::new())),
      functions: Arc::new(Mutex::new(HashMap::new())),
    }
  }
}

impl Default for Context {
  fn default() -> Self {
    Self::new()
  }
}

impl Clone for Context {
  fn clone(&self) -> Self {
    Context {
      constants: self.constants.clone(),
      variables: self.variables.clone(),
      functions: self.functions.clone(),
    }
  }
}

impl PartialEq for Context {
  fn eq(&self, other: &Self) -> bool {
    Arc::ptr_eq(&self.constants, &other.constants)
      && Arc::ptr_eq(&self.variables, &other.variables)
      && Arc::ptr_eq(&self.functions, &other.functions)
  }
}

impl Eq for Context {}
