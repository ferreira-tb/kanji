use crate::database::sql_types::{KanjiSetChunkId, Version, Zoned};
use bon::Builder;
use diesel::prelude::*;
use serde::Serialize;

#[cfg(desktop)]
use {crate::manager::ManagerExt, anyhow::Result, tauri::AppHandle};

#[derive(Identifiable, Queryable, Selectable, Clone, Debug, Serialize)]
#[diesel(table_name = crate::database::schema::quiz_chunk_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct QuizChunkHistoryEntry {
  pub id: KanjiSetChunkId,
  pub last_quiz: Zoned,
  pub last_quiz_version: Version,
}

#[derive(Insertable, Builder, Clone, Debug)]
#[diesel(table_name = crate::database::schema::quiz_chunk_history)]
pub struct NewQuizChunkHistoryEntry {
  #[builder(start_fn)]
  id: KanjiSetChunkId,

  #[builder(default)]
  pub last_quiz: Zoned,

  #[builder(default)]
  pub last_quiz_version: Version,
}

#[cfg(desktop)]
impl NewQuizChunkHistoryEntry {
  pub fn create(self, app: &AppHandle) -> Result<()> {
    app
      .database()
      .create_quiz_chunk_history_entry(&self)
  }
}
