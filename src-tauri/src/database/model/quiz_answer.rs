use crate::database::sql_types::{KanjiChar, QuizAnswerId, SourceId, Zoned};
use anyhow::Result;
use bon::Builder;
use diesel::prelude::*;
use serde::Serialize;
use tauri::AppHandle;

#[cfg(desktop)]
use crate::manager::ManagerExt;

#[derive(Identifiable, Queryable, Selectable, Clone, Debug, Serialize)]
#[diesel(table_name = crate::database::schema::quiz_answer)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct QuizAnswer {
  pub id: QuizAnswerId,
  pub question: KanjiChar,
  pub answer: KanjiChar,
  pub created_at: Zoned,
  pub source_id: Option<SourceId>,
}

#[derive(Insertable, Builder, Clone, Debug)]
#[diesel(table_name = crate::database::schema::quiz_answer)]
pub struct NewQuizAnswer {
  #[builder(skip)]
  id: QuizAnswerId,

  #[builder(into)]
  question: KanjiChar,

  #[builder(into)]
  answer: KanjiChar,

  #[builder(skip = Zoned::now())]
  created_at: Zoned,

  #[builder(into)]
  source_id: Option<SourceId>,
}

#[cfg(desktop)]
impl NewQuizAnswer {
  pub fn create(self, app: &AppHandle) -> Result<QuizAnswerId> {
    app.database().create_quiz_answer(&self)
  }
}
