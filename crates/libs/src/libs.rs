pub mod io;
pub mod math;
pub mod parse;

use std::{
  sync::Arc,
  time::{Duration, Instant},
};

use math::*;
use parking_lot::RwLock;
use parse::*;

use ::enums::{Data, Expr};
use once_cell::sync::Lazy;
use tauri::AppHandle;
use thiserror::Error;

pub fn load(
  name: &str,
) -> Option<
  Vec<(
    &str,
    fn(Vec<Expr>, &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data>,
  )>,
> {
  match name {
    "matematica" => Some(vec![("raiz", square_root), ("potencia", power), ("PI", pi)]),
    "conversao" => Some(vec![("int", int), ("real", float)]),
    _ => None,
  }
}

pub fn get_lib_funcs(name: &str) -> Vec<&'static str> {
  match name {
    "matematica" => vec!["raiz", "potencia", "PI"],
    "conversao" => vec!["int", "real"],
    _ => vec![],
  }
}

pub fn has(name: &str) -> bool {
  match name {
    "escrever" | "erro" | "ler" | "raiz" | "potencia" | "int" | "real" => true,
    _ => false,
  }
}

#[derive(Debug, Error)]
pub enum AppHandleError {
  #[error("App handle not initialized")]
  NotInitialized,
  #[error("App handle already initialized")]
  AlreadyInitialized,
}

type Result<T> = std::result::Result<T, AppHandleError>;

static APP_HANDLE: Lazy<Arc<RwLock<Option<AppHandle>>>> = Lazy::new(|| Arc::new(RwLock::new(None)));

pub fn init_app_handle(handle: AppHandle) -> Result<()> {
  let mut app_handle = APP_HANDLE.write();
  if app_handle.is_some() {
    return Err(AppHandleError::AlreadyInitialized);
  }
  *app_handle = Some(handle);
  Ok(())
}

pub fn get_app_handle() -> Result<AppHandle> {
  APP_HANDLE
    .read()
    .as_ref()
    .cloned()
    .ok_or(AppHandleError::NotInitialized)
}

// Optional: Create a wrapper struct for more complex operations
pub struct AppHandleManager;

impl AppHandleManager {
  // Initialize with custom options
  pub fn init(handle: AppHandle) -> Result<()> {
    let mut app_handle = APP_HANDLE.write();
    if app_handle.is_some() {
      return Err(AppHandleError::AlreadyInitialized);
    }

    // Apply options before storing
    *app_handle = Some(handle);
    Ok(())
  }

  // Safe read access with timeout
  pub fn try_get_handle_timeout(timeout: Duration) -> Result<AppHandle> {
    let start = Instant::now();
    while start.elapsed() < timeout {
      if let Some(handle) = APP_HANDLE.read().as_ref().cloned() {
        return Ok(handle);
      }
      std::thread::sleep(Duration::from_millis(10));
    }
    Err(AppHandleError::NotInitialized)
  }

  // Modify handle with existing lock
  pub fn with_handle<F, T>(&self, f: F) -> Result<T>
  where
    F: FnOnce(&AppHandle) -> T,
  {
    let handle = APP_HANDLE.read();
    handle.as_ref().map(f).ok_or(AppHandleError::NotInitialized)
  }

  // Update handle safely
  pub fn update_handle<F>(&self, f: F) -> Result<()>
  where
    F: FnOnce(&mut AppHandle),
  {
    let mut handle = APP_HANDLE.write();
    if let Some(ref mut h) = *handle {
      f(h);
      Ok(())
    } else {
      Err(AppHandleError::NotInitialized)
    }
  }

  pub fn get_handle(&self) -> Result<AppHandle> {
    APP_HANDLE
      .read()
      .as_ref()
      .cloned()
      .ok_or(AppHandleError::NotInitialized)
  }
}
