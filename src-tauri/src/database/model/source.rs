use crate::database::sql_types::{SourceId, SourceWeight, SqlPath, Zoned};
use bon::Builder;
use diesel::prelude::*;
use serde::Serialize;

#[cfg(desktop)]
use {crate::manager::ManagerExt, anyhow::Result, tauri::AppHandle};

#[derive(Identifiable, Queryable, Selectable, Clone, Debug, Serialize)]
#[diesel(table_name = crate::database::schema::source)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Source {
  pub id: SourceId,
  pub path: SqlPath,
  pub name: String,
  pub created_at: Zoned,
  pub updated_at: Zoned,
  pub enabled: bool,
  pub weight: SourceWeight,
}

#[derive(Insertable, Builder, Clone, Debug)]
#[diesel(table_name = crate::database::schema::source)]
pub struct NewSource<'a> {
  #[builder(start_fn, into)]
  path: SqlPath,

  #[builder(into)]
  name: &'a str,

  #[builder(skip = Zoned::now())]
  created_at: Zoned,

  #[builder(skip = Zoned::now())]
  updated_at: Zoned,

  #[builder(skip)]
  weight: SourceWeight,
}

#[cfg(desktop)]
impl NewSource<'_> {
  pub fn create(self, app: &AppHandle) -> Result<SourceId> {
    app.database().create_source(&self)
  }
}
