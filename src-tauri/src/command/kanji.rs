use crate::core::kanji::{self, KanjiStats};
use crate::core::set::KanjiSet;
use crate::core::snippet::{self, Snippet};
use crate::database::sql_types::{KanjiChar, SourceId};
use crate::error::CResult;
use tauri::AppHandle;

#[tauri::command]
pub async fn export_set(app: AppHandle) -> CResult<()> {
  if let Some(folder) = super::pick_folders(app.clone())
    .await?
    .first()
  {
    KanjiSet::load(&app)
      .await?
      .export(&app, folder)
      .await?;
  }

  Ok(())
}

#[tauri::command]
pub async fn get_set(app: AppHandle) -> CResult<KanjiSet> {
  KanjiSet::load(&app)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn search_kanji(app: AppHandle) -> CResult<Vec<KanjiStats>> {
  kanji::search(app).await.map_err(Into::into)
}

#[tauri::command]
pub async fn search_snippets(
  app: AppHandle,
  kanji: KanjiChar,
  source: Option<SourceId>,
) -> CResult<Vec<Snippet>> {
  snippet::search(app, kanji, source)
    .await
    .map_err(Into::into)
}
