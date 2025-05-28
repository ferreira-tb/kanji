#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
#![feature(try_blocks)]

mod api;
mod command;
mod error;
mod kanji;
mod plugin;
mod tray;
mod window;

use error::BoxResult;
use tauri::{AppHandle, Manager};

fn main() {
  let specta = api::collect();
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
    .invoke_handler(specta.invoke_handler())
    .run(tauri::generate_context!())
    .expect("failed to start tauri app");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  window::open(app)?;
  Ok(())
}
