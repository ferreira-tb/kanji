#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(file_buffered, try_blocks)]

mod command;
mod error;
mod kanji;
mod plugin;
mod snippet;
mod tray;
mod util;
mod window;

use error::BoxResult;
use tauri::{AppHandle, Manager};

fn main() {
  tauri::Builder::default()
    .plugin(plugin::prevent_default())
    .plugin(plugin::single_instance())
    .plugin(plugin::window_state())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_persisted_scope::init())
    .plugin(tauri_plugin_pinia::init())
    .plugin(tauri_plugin_process::init())
    .setup(|app| setup(app.app_handle()))
    .invoke_handler(tauri::generate_handler![
      command::create_tray_icon,
      command::open,
      command::pick_folder,
      command::search_kanji,
      command::search_snippets,
      command::show_window
    ])
    .run(tauri::generate_context!())
    .expect("failed to start tauri app");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  window::open(app)?;
  Ok(())
}
