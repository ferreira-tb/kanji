use crate::error::CResult;
use crate::kanji::{self, Kanji};
use crate::tray;
use std::path::PathBuf;
use tauri::{AppHandle, WebviewWindow};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::{FilePath, FsExt};
use tokio::sync::oneshot;

#[tauri::command]
pub async fn create_tray_icon(app: AppHandle) -> CResult<()> {
  let handle = app.clone();
  handle
    .run_on_main_thread(move || tray::create(&app).unwrap())
    .map_err(Into::into)
}

#[tauri::command]
pub async fn pick_folder(app: AppHandle) -> CResult<Option<PathBuf>> {
  let (tx, rx) = oneshot::channel();
  app.dialog().file().pick_folder(move |response| {
    let _ = tx.send(response.map(FilePath::into_path));
  });

  let path = rx.await?.transpose()?;
  if let Some(path) = path.as_deref() {
    let scope = app.fs_scope();
    if !scope.is_allowed(path) {
      let _ = scope.allow_directory(path, true);
    }
  }

  Ok(path)
}

#[tauri::command]
pub async fn search_kanji(path: PathBuf) -> CResult<Vec<Kanji>> {
  kanji::search(path).await.map_err(Into::into)
}

#[tauri::command]
pub async fn show_window(window: WebviewWindow) -> CResult<()> {
  window
    .show()
    .and_then(|()| window.unminimize())
    .and_then(|()| window.set_focus())
    .map_err(Into::into)
}
