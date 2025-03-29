use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::sync::Arc;

use crate::event::GLOBAL_EVENT_SYSTEM;

use super::Payload;

#[derive(Clone)]
pub struct LogBatchConfig {
  pub process_threshold: usize,
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
    let batch_data_str =
      serde_json::to_string(&batch_data).expect("Failed to serialize batch data");
    GLOBAL_EVENT_SYSTEM.emit("process_logs", batch_data_str);

    self.clear();
  }
}
