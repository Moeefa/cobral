mod commands;

use libs::APP_HANDLE;
use tauri::Manager;
use tauri_plugin_decorum::WebviewWindowExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_decorum::init())
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![
      commands::parse,
      commands::step,
      commands::update
    ])
    .setup(|app| {
      APP_HANDLE.lock().unwrap().replace(app.handle().clone());

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
        main_window.set_window_level(25).unwrap()
      }

      main_window.show().unwrap();

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
