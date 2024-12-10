use std::sync::{Arc, LazyLock, Mutex};

use tauri::{command, AppHandle, Runtime, Window};

use crate::context::Context;

static CONTEXT: LazyLock<Arc<Mutex<Context>>> =
  LazyLock::new(|| Arc::new(Mutex::new(Context::new())));

#[command]
pub async fn update<R: Runtime>(_app: AppHandle<R>, _window: Window<R>, input: String) {
  CONTEXT
    .lock()
    .unwrap_or_else(|e| e.into_inner())
    .update(input);
}

#[command]
pub async fn parse<R: Runtime>(app: AppHandle<R>, _window: Window<R>, input: String) {
  CONTEXT
    .lock()
    .unwrap_or_else(|e| e.into_inner())
    .parse(app, input);
}
