mod context;
mod utils;

use cobral::{event::GLOBAL_EVENT_SYSTEM, logger};
use context::ExecutionContext;
use logger::batcher::{LogBatchConfig, LogBatchManager};
use tauri::{
  window::{Effect, EffectsBuilder},
  Emitter, Listener, Manager,
};
use tauri_plugin_decorum::WebviewWindowExt;
use utils::AppHandleManager;

#[tauri::command]
async fn eval(context: tauri::State<'_, ExecutionContext>, input: String) -> Result<(), String> {
  context.eval(input).await;
  Ok(())
}

pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_decorum::init())
    .plugin(tauri_plugin_shell::init())
    .manage(ExecutionContext::new())
    .invoke_handler(tauri::generate_handler![eval])
    .setup(|app| {
      AppHandleManager::init(app.handle().clone());
      LogBatchManager::init(LogBatchConfig::default());

      app
        .handle()
        .plugin(tauri_plugin_updater::Builder::new().build())?;

      app.listen("break_exec", |_| {
        GLOBAL_EVENT_SYSTEM.emit("break_exec", "".to_string());
      });

      app.listen("submit_pending_input", |event| {
        GLOBAL_EVENT_SYSTEM.emit("submit_pending_input", event.payload().to_owned());
      });

      GLOBAL_EVENT_SYSTEM.listen(
        "process_logs",
        Box::new(|batch_data| {
          let _ = AppHandleManager.with_handle(|handle| {
            let data: serde_json::Value =
              serde_json::from_str(&batch_data).expect("failed to deserialize batch data");
            handle
              .emit("process_logs", data)
              .expect("failed to emit log_batch event");
          });
        }),
      );

      GLOBAL_EVENT_SYSTEM.listen(
        "spawn_input",
        Box::new(|input| {
          let _ = AppHandleManager.with_handle(|handle| {
            handle
              .emit("spawn_input", Some(input))
              .expect("failed to emit read_input event");
          });
        }),
      );

      let main_window_builder =
        tauri::WebviewWindowBuilder::new(app.handle(), "main", tauri::WebviewUrl::App("/".into()))
          .title("Cobral")
          .resizable(true)
          .decorations(false)
          .inner_size(1000.0, 562.0)
          .min_inner_size(500.0, 300.0)
          .center()
          .effects(
            EffectsBuilder::new()
              .effects([Effect::Acrylic, Effect::Sidebar])
              .build(),
          );

      #[cfg(target_os = "windows")]
      let main_window_builder = main_window_builder.transparent(true);

      main_window_builder.build().unwrap();

      // Create a custom titlebar for main window
      // On Windows this hides decoration and creates custom window controls
      // On macOS it needs hiddenTitle: true and titleBarStyle: overlay
      let main_window = app.get_webview_window("main").unwrap();
      main_window.create_overlay_titlebar().unwrap();

      // Some macOS-specific helpers
      #[cfg(target_os = "macos")]
      {
        // Set a custom inset to the traffic lights
        main_window.set_traffic_lights_inset(12.0, 16.0).unwrap();

        // Make window transparent without privateApi
        main_window.make_transparent().unwrap();

        // Set window level
        // NSWindowLevel: https://developer.apple.com/documentation/appkit/nswindowlevel
        main_window.set_window_level(25).unwrap();
      }

      main_window.show().unwrap();

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
