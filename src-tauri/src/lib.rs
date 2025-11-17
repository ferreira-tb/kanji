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
  let builder = tauri::Builder::default()
    .plugin(plugin::prevent_default())
    .plugin(plugin::single_instance())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .invoke_handler(tauri::generate_handler![
      command::create_tray_icon,
      command::get_server_addr,
      command::open_editor,
      command::pick_folders,
      command::show_window,
      command::bookmark::create_bookmark,
      command::bookmark::get_bookmarks,
      command::bookmark::remove_bookmark,
      command::kanji::export_set,
      command::kanji::get_set,
      command::kanji::search_kanji,
      command::kanji::search_snippets,
      command::quiz::create_quiz,
      command::quiz::create_quiz_answer,
      command::quiz::get_quiz_answers,
      command::quiz::get_quiz_source_stats,
      command::source::create_source,
      command::source::get_source,
      command::source::get_sources,
      command::source::rename_source,
      command::source::remove_source,
      command::source::set_source_weight,
      command::source::toggle_source,
      command::source_group::create_source_group,
      command::source_group::get_source_group,
      command::source_group::get_source_group_ids,
      command::source_group::get_source_group_source_ids,
      command::source_group::get_source_group_sources,
      command::source_group::get_source_groups,
      command::source_group::remove_source_group,
      command::source_group::rename_source_group,
      command::source_group::set_source_group_sources
    ]);

  #[cfg(mobile)]
  let builder = tauri::Builder::default();

  #[cfg(debug_assertions)]
  let builder = builder.plugin(plugin::log());

  builder
    .plugin(plugin::pinia())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_persisted_scope::init())
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_http::init())
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
