use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::Emitter;

use crate::Payload;

#[derive(Debug, thiserror::Error)]
pub enum LogBatchError {
  #[error("Failed to write to batch")]
  WriteError,
  #[error("Batch locked for too long")]
  LockTimeout,
}

type Result<T> = std::result::Result<T, LogBatchError>;

#[derive(Clone)]
pub struct LogBatchConfig {
  process_threshold: usize,
  lock_timeout: Duration,
}

impl Default for LogBatchConfig {
  fn default() -> Self {
    Self {
      process_threshold: 1000,
      lock_timeout: Duration::from_secs(1),
    }
  }
}

static LOG_BATCH: Lazy<Arc<RwLock<Vec<Payload>>>> = Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

static BATCH_CONFIG: Lazy<RwLock<LogBatchConfig>> =
  Lazy::new(|| RwLock::new(LogBatchConfig::default()));

static APP_HANDLE: Lazy<Arc<RwLock<Option<tauri::AppHandle<tauri::Wry>>>>> =
  Lazy::new(|| Arc::new(RwLock::new(None)));

pub struct LogBatchManager;

impl LogBatchManager {
  pub fn init(config: LogBatchConfig, app_handle: tauri::AppHandle<tauri::Wry>) {
    *BATCH_CONFIG.write() = config;
    *APP_HANDLE.write() = Some(app_handle);
  }

  pub fn is_empty(&self) -> bool {
    let batch = LOG_BATCH.read();
    batch.is_empty()
  }

  pub fn is_threshold_reached(&self) -> bool {
    let batch = LOG_BATCH.read();
    let config = BATCH_CONFIG.read();
    batch.len() >= config.process_threshold
  }

  pub fn clear(&self) {
    let mut batch = LOG_BATCH.write();
    batch.clear();
  }

  pub fn add(&self, payload: Payload) -> Result<()> {
    let config = BATCH_CONFIG.read();
    let mut batch = LOG_BATCH.write();

    batch.push(payload);

    if batch.len() >= config.process_threshold {
      drop(batch);
      self.process_batch();
    }

    Ok(())
  }

  pub fn add_multiple(&self, payloads: Vec<Payload>) -> Result<()> {
    let config = BATCH_CONFIG.read();
    let mut batch = LOG_BATCH.write();

    batch.extend(payloads);

    if batch.len() >= config.process_threshold {
      drop(batch);
      self.process_batch();
    }

    Ok(())
  }

  pub fn get_all(&self) -> Result<Vec<Payload>> {
    let start = Instant::now();
    let config = BATCH_CONFIG.read();

    while start.elapsed() < config.lock_timeout {
      if let Some(batch) = LOG_BATCH.try_read() {
        return Ok(batch.clone());
      }
      std::thread::sleep(Duration::from_millis(10));
    }

    Err(LogBatchError::LockTimeout)
  }

  pub fn process_batch(&self) {
    let batch_data = LOG_BATCH.read().clone();
    let app_handle = APP_HANDLE.read();

    if let Some(handle) = app_handle.as_ref() {
      handle
        .emit("log_batch", batch_data)
        .expect("Failed to emit log event");
    }

    self.clear();
  }

  pub fn get_stats(&self) -> Result<BatchStats> {
    let batch = LOG_BATCH.read();
    let config = BATCH_CONFIG.read();

    Ok(BatchStats {
      current_size: batch.len(),
      capacity: batch.capacity(),
      process_threshold: config.process_threshold,
    })
  }
}

#[derive(Debug, Clone)]
pub struct BatchStats {
  pub current_size: usize,
  pub capacity: usize,
  pub process_threshold: usize,
}
