// global_event.rs
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

use rand::Rng;

use crate::interpreter::error::InterpreterError;
use crate::interpreter::value::Value;

pub static GLOBAL_EVENT_SYSTEM: LazyLock<GlobalEventSystem> =
  LazyLock::new(|| GlobalEventSystem::new());

pub struct GlobalEventSystem {
  listeners: Arc<Mutex<HashMap<String, Vec<(usize, Box<dyn Fn(String) + Send + Sync>)>>>>,
  callbacks:
    Arc<Mutex<HashMap<u32, Box<dyn Fn(String) -> Result<Value, InterpreterError> + Send + Sync>>>>,
}

impl GlobalEventSystem {
  pub fn new() -> Self {
    GlobalEventSystem {
      listeners: Arc::new(Mutex::new(HashMap::new())),
      callbacks: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  pub fn emit(&self, event: &str, payload: String) {
    if let Some(listeners) = self.listeners.lock().unwrap().get(event) {
      for (_, listener) in listeners {
        listener(payload.clone());
      }
    }
  }

  pub fn listen(&self, event: &str, callback: Box<dyn Fn(String) + Send + Sync>) -> usize {
    let mut rng = rand::rng();
    let id: u32 = rng.random();
    self
      .listeners
      .lock()
      .unwrap()
      .entry(event.to_string())
      .or_insert_with(Vec::new)
      .push((id.try_into().unwrap(), callback));
    id.try_into().unwrap()
  }

  pub fn unlisten(&self, id: usize) {
    let mut listeners = self.listeners.lock().unwrap();
    for (_, event_listeners) in listeners.iter_mut() {
      if let Some(index) = event_listeners
        .iter()
        .position(|(listener_id, _)| *listener_id == id)
      {
        #[allow(unused_must_use)]
        event_listeners.remove(index);
      }
    }
  }

  pub fn register_callback(
    &self,
    callback: Box<dyn Fn(String) -> Result<Value, InterpreterError> + Send + Sync>,
  ) -> u32 {
    let mut rng = rand::rng();
    let id: u32 = rng.random();

    // Store the callback
    let mut callbacks = self.callbacks.lock().unwrap();
    callbacks.insert(id, callback);

    id
  }

  // Add a method to resolve callbacks
  pub fn resolve_callback(
    &self,
    id: &u32,
    input: String,
  ) -> Option<Result<Value, InterpreterError>> {
    let callbacks = self.callbacks.lock().unwrap();
    if let Some(callback) = callbacks.get(id) {
      Some(callback(input))
    } else {
      None
    }
  }
}
