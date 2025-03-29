use std::sync::Arc;

use parking_lot::RwLock;

use once_cell::sync::Lazy;
use tauri::AppHandle;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppHandleError {
  #[error("App handle not initialized")]
  NotInitialized,
}

type Result<T> = std::result::Result<T, AppHandleError>;

static APP_HANDLE: Lazy<Arc<RwLock<Option<AppHandle>>>> = Lazy::new(|| Arc::new(RwLock::new(None)));

// Optional: Create a wrapper struct for more complex operations
pub struct AppHandleManager;

impl AppHandleManager {
  // Initialize with custom options
  pub fn init(handle: AppHandle) {
    let mut app_handle = APP_HANDLE.write();

    // Apply options before storing
    *app_handle = Some(handle);
  }

  // Modify handle with existing lock
  pub fn with_handle<F, T>(&self, f: F) -> Result<T>
  where
    F: FnOnce(&AppHandle) -> T,
  {
    let handle = APP_HANDLE.read();
    handle.as_ref().map(f).ok_or(AppHandleError::NotInitialized)
  }

  pub fn get_handle(&self) -> Result<AppHandle> {
    APP_HANDLE
      .read()
      .as_ref()
      .cloned()
      .ok_or(AppHandleError::NotInitialized)
  }
}
