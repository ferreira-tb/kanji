#![feature(file_buffered, try_blocks)]

mod database;
mod error;
mod kanji;
mod plugin;
mod quiz;
mod set;
mod settings;
mod snippet;
mod window;

#[cfg(desktop)]
mod command;
#[cfg(desktop)]
mod manager;
#[cfg(desktop)]
mod server;
#[cfg(desktop)]
mod tray;

use error::BoxResult;
use tauri::{AppHandle, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  #[cfg(desktop)]
  let builder = {
    tauri::Builder::default()
      .plugin(plugin::prevent_default())
      .plugin(plugin::single_instance())
      .plugin(plugin::window_state())
      .plugin(tauri_plugin_updater::Builder::new().build())
      .invoke_handler(tauri::generate_handler![
        command::create_bookmark,
        command::create_quiz,
        command::create_quiz_answer,
        command::create_random_quiz,
        command::create_source,
        command::create_tray_icon,
        command::export_set,
        command::get_bookmarks,
        command::get_quiz_answers,
        command::get_server_addr,
        command::get_set,
        command::get_sources,
        command::open,
        command::pick_folders,
        command::rename_source,
        command::remove_bookmark,
        command::remove_source,
        command::search_kanji,
        command::search_snippets,
        command::set_source_weight,
        command::show_window,
        command::toggle_source
      ])
  };

  #[cfg(mobile)]
  let builder = tauri::Builder::default();

  builder
    .plugin(plugin::pinia())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_persisted_scope::init())
    .plugin(tauri_plugin_process::init())
    .setup(|app| setup(app.app_handle()))
    .run(tauri::generate_context!())
    .expect("failed to start tauri app");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  #[cfg(desktop)]
  {
    use database::DatabaseHandle;
    use server::Server;

    app.manage(DatabaseHandle::new()?);
    app.manage(Server::serve(app)?);
  }

  #[cfg(desktop)]
  window::desktop::open(app)?;
  #[cfg(mobile)]
  window::mobile::open(app)?;

  Ok(())
}
