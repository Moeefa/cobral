mod context;

use context::ExecutionContext;
use libs::APP_HANDLE;
use tauri::{
  window::{Effect, EffectsBuilder},
  Manager,
};
use tauri_plugin_decorum::WebviewWindowExt;

#[tauri::command]
async fn eval(context: tauri::State<'_, ExecutionContext>, input: String) -> Result<(), String> {
  context.eval(input).await;
  Ok(())
}

pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_decorum::init())
    .plugin(tauri_plugin_shell::init())
    .manage(ExecutionContext::new())
    .invoke_handler(tauri::generate_handler![eval])
    .setup(|app| {
      APP_HANDLE.lock().unwrap().replace(app.handle().clone());

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
              .effects([Effect::Acrylic, Effect::FullScreenUI])
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
