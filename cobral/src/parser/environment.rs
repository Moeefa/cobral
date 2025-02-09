use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};

use crate::shared::ast::Expression;

pub struct Environment {
  pub constants: Arc<RwLock<HashMap<String, Option<Expression>>>>,
  pub variables: Arc<RwLock<HashMap<String, Option<Expression>>>>,
  pub functions: Arc<RwLock<HashMap<String, Option<Vec<String>>>>>,
  pub libs: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl Environment {
  pub fn new() -> Self {
    Self {
      constants: Arc::new(RwLock::new(HashMap::new())),
      variables: Arc::new(RwLock::new(HashMap::new())),
      functions: Arc::new(RwLock::new(HashMap::new())),
      libs: Arc::new(RwLock::new(Self::default_libs())),
    }
  }

  fn default_libs() -> HashMap<String, Vec<String>> {
    HashMap::from([(
      "io".to_string(),
      vec![
        "escrever".to_string(),
        "erro".to_string(),
        "ler".to_string(),
      ],
    )])
  }
}
