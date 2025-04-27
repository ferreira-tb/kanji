use crate::error::CResult;

#[cfg(desktop)]
use tauri::WebviewWindow;

#[tauri::command]
#[specta::specta]
pub async fn is_desktop() -> bool {
  cfg!(desktop)
}

#[tauri::command]
#[specta::specta]
pub async fn is_mobile() -> bool {
  cfg!(mobile)
}

#[cfg(desktop)]
#[tauri::command]
#[specta::specta]
pub async fn show_window(window: WebviewWindow) -> CResult<()> {
  window
    .show()
    .and_then(|()| window.unminimize())
    .and_then(|()| window.set_focus())
    .map_err(Into::into)
}

#[cfg(mobile)]
#[tauri::command]
#[specta::specta]
pub async fn show_window() -> CResult<()> {
  Ok(())
}
