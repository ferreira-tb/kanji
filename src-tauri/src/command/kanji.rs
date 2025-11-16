use crate::database::sql_types::{KanjiChar, SourceId};
use crate::error::CResult;
use crate::kanji::{self, KanjiStats};
use crate::set::KanjiSet;
use crate::snippet::{self, Snippet};
use tauri::AppHandle;

#[tauri::command]
pub async fn export_set(app: AppHandle) -> CResult<()> {
  if let Some(folder) = super::pick_folders(app.clone())
    .await?
    .first()
  {
    KanjiSet::load(app.clone())
      .await?
      .export(app, folder)
      .await?;
  }

  Ok(())
}

#[tauri::command]
pub async fn get_set(app: AppHandle) -> CResult<KanjiSet> {
  KanjiSet::load(app).await.map_err(Into::into)
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
