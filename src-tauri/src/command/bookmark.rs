use crate::database::model::bookmark::Bookmark;
use crate::database::sql_types::BookmarkId;
use crate::error::CResult;
use crate::manager::ManagerExt;
use crate::snippet::Snippet;
use tauri::AppHandle;

#[tauri::command]
pub async fn create_bookmark(app: AppHandle, snippet: Snippet) -> CResult<BookmarkId> {
  snippet
    .create_bookmark(&app)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_bookmarks(app: AppHandle) -> CResult<Vec<Bookmark>> {
  app
    .database()
    .get_bookmarks()
    .map_err(Into::into)
}

#[tauri::command]
pub async fn remove_bookmark(app: AppHandle, id: BookmarkId) -> CResult<usize> {
  app
    .database()
    .remove_bookmark(id)
    .map_err(Into::into)
}
