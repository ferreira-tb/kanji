use crate::database::sql_types::{KanjiChar, Zoned};
use crate::manager::ManagerExt;
use anyhow::Result;
use bon::Builder;
use diesel::prelude::*;
use serde::Serialize;
use tauri::AppHandle;

#[derive(Identifiable, Queryable, Selectable, Clone, Debug, Serialize)]
#[diesel(table_name = crate::database::schema::kanji)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Kanji {
  pub id: KanjiChar,
  pub created_at: Zoned,
  pub updated_at: Zoned,
}

#[derive(Insertable, Builder, Clone, Debug)]
#[diesel(table_name = crate::database::schema::kanji)]
pub struct NewKanji {
  #[builder(start_fn)]
  id: KanjiChar,

  #[builder(skip = Zoned::now())]
  created_at: Zoned,

  #[builder(skip = Zoned::now())]
  updated_at: Zoned,
}

impl NewKanji {
  pub fn create(self, app: &AppHandle) -> Result<()> {
    app.database().create_kanji(&self)
  }
}
