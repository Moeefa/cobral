use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::sync::Arc;
use tauri::Emitter;

use crate::shared::AppHandleManager;

use super::Payload;

#[derive(Clone)]
pub struct LogBatchConfig {
  process_threshold: usize,
}

impl Default for LogBatchConfig {
  fn default() -> Self {
    Self {
      process_threshold: 1000,
    }
  }
}

static LOG_BATCH: Lazy<Arc<RwLock<Vec<Payload>>>> = Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

static BATCH_CONFIG: Lazy<RwLock<LogBatchConfig>> =
  Lazy::new(|| RwLock::new(LogBatchConfig::default()));

pub struct LogBatchManager;

impl LogBatchManager {
  pub fn init(config: LogBatchConfig) {
    *BATCH_CONFIG.write() = config;
  }

  pub fn clear(&self) {
    let mut batch = LOG_BATCH.write();
    batch.clear();
  }

  pub fn add(&self, payload: Payload) {
    let config = BATCH_CONFIG.read();
    let mut batch = LOG_BATCH.write();

    batch.push(payload);

    if batch.len() >= config.process_threshold {
      drop(batch);
      self.process_batch();
    }
  }

  pub fn process_batch(&self) {
    let batch_data = LOG_BATCH.read().clone();
    let _ = AppHandleManager.with_handle(|handle| {
      handle
        .emit("log_batch", batch_data)
        .expect("Failed to emit log event");
    });

    self.clear();
  }
}
