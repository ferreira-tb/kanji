use crate::error::CResult;
use crate::kanji::{self, Frequency};
use std::path::PathBuf;
use tauri::{AppHandle, WebviewWindow};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::FilePath;
use tokio::sync::oneshot;

#[tauri::command]
#[specta::specta]
pub async fn pick_folder(app: AppHandle) -> CResult<Option<PathBuf>> {
  let (tx, rx) = oneshot::channel();
  app.dialog().file().pick_folder(move |response| {
    let _ = tx.send(response);
  });

  rx.await?
    .map(FilePath::into_path)
    .transpose()
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn search_kanji(path: PathBuf) -> CResult<Vec<Frequency>> {
  kanji::search(path).await.map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn show_window(window: WebviewWindow) -> CResult<()> {
  window
    .show()
    .and_then(|()| window.unminimize())
    .and_then(|()| window.set_focus())
    .map_err(Into::into)
}
