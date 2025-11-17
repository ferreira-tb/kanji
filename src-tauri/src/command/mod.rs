pub mod bookmark;
pub mod kanji;
pub mod quiz;
pub mod source;
pub mod source_group;

use crate::error::CResult;
use crate::manager::ManagerExt;
use crate::settings::Editor;
use crate::tray;
use itertools::Itertools;
use std::net::SocketAddrV4;
use std::path::PathBuf;
use std::process::Stdio;
use tauri::{AppHandle, WebviewWindow};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::{FilePath, FsExt};
use tokio::process::Command;
use tokio::sync::oneshot;
use windows::Win32::System::Threading::{CREATE_NEW_PROCESS_GROUP, CREATE_NO_WINDOW};

#[tauri::command]
pub async fn create_tray_icon(app: AppHandle) -> CResult<()> {
  let handle = app.clone();
  handle
    .run_on_main_thread(move || tray::create(&app).unwrap())
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_server_addr(app: AppHandle) -> SocketAddrV4 {
  app.server().addr()
}

#[tauri::command]
pub async fn open_editor(app: AppHandle, path: PathBuf, line: u32) -> CResult<()> {
  let path = format!("{}:{}", path.to_string_lossy(), line);
  let args: &[&str] = match Editor::get(&app) {
    Editor::Code => &[Editor::Code.as_ref(), "--goto", path.as_str()],
    Editor::CodeInsiders => &[Editor::CodeInsiders.as_ref(), "--goto", path.as_str()],
    Editor::Zed => &[Editor::Zed.as_ref(), path.as_str()],
  };

  Command::new("pwsh")
    .arg("-Command")
    .args(args)
    .creation_flags(CREATE_NEW_PROCESS_GROUP.0 | CREATE_NO_WINDOW.0)
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()
    .await?;

  Ok(())
}

#[tauri::command]
pub async fn pick_folders(app: AppHandle) -> CResult<Vec<PathBuf>> {
  let (tx, rx) = oneshot::channel();
  app
    .dialog()
    .file()
    .pick_folders(move |response| {
      let folders = response
        .unwrap_or_default()
        .into_iter()
        .map(FilePath::into_path)
        .process_results(|it| it.collect_vec());

      let _ = tx.send(folders);
    });

  let folders = rx.await??;
  for folder in &folders {
    let scope = app.fs_scope();
    if !scope.is_allowed(folder) {
      let _ = scope.allow_directory(folder, true);
    }
  }

  Ok(folders)
}

#[tauri::command]
pub async fn show_window(window: WebviewWindow) -> CResult<()> {
  window
    .show()
    .and_then(|()| window.unminimize())
    .and_then(|()| window.set_focus())
    .map_err(Into::into)
}
