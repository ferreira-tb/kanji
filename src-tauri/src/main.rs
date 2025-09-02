#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(file_buffered, try_blocks)]

mod command;
mod database;
mod error;
mod kanji;
mod manager;
mod plugin;
mod quiz;
mod set;
mod settings;
mod snippet;
mod tray;
mod window;

use crate::database::DatabaseHandle;
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
    .plugin(tauri_plugin_updater::Builder::new().build())
    .setup(|app| setup(app.app_handle()))
    .invoke_handler(tauri::generate_handler![
      command::create_quiz_answer,
      command::create_source,
      command::create_tray_icon,
      command::export_set,
      command::get_set,
      command::get_sources,
      command::open,
      command::pick_folders,
      command::rename_source,
      command::search_kanji,
      command::search_snippets,
      command::show_window,
      command::start_quiz
    ])
    .run(tauri::generate_context!())
    .expect("failed to start tauri app");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  app.manage(DatabaseHandle::new()?);
  window::open(app)?;
  Ok(())
}
