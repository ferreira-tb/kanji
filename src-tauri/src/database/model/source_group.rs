use crate::database::model::source::Source;
use crate::database::sql_types::{SourceGroupId, SourceId, Zoned};
use bon::Builder;
use diesel::prelude::*;
use serde::Serialize;

#[cfg(desktop)]
use {crate::manager::ManagerExt, anyhow::Result, tauri::AppHandle};

#[derive(Identifiable, Queryable, Selectable, Clone, Debug, Serialize)]
#[diesel(table_name = crate::database::schema::source_group)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct SourceGroup {
  pub id: SourceGroupId,
  pub name: String,
  pub created_at: Zoned,
  pub updated_at: Zoned,
}

#[derive(Insertable, Builder, Clone, Debug)]
#[diesel(table_name = crate::database::schema::source_group)]
pub struct NewSourceGroup<'a> {
  #[builder(into)]
  name: &'a str,

  #[builder(skip = Zoned::now())]
  created_at: Zoned,

  #[builder(skip = Zoned::now())]
  updated_at: Zoned,
}

#[cfg(desktop)]
impl NewSourceGroup<'_> {
  pub fn create(self, app: &AppHandle) -> Result<SourceGroupId> {
    app.database().create_source_group(&self)
  }
}

#[derive(Associations, Identifiable, Insertable, Selectable, Queryable, Debug)]
#[diesel(table_name = crate::database::schema::source_group_source)]
#[diesel(belongs_to(SourceGroup, foreign_key = source_group_id))]
#[diesel(belongs_to(Source, foreign_key = source_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(source_group_id, source_id))]
pub struct SourceGroupSource {
  pub source_group_id: SourceGroupId,
  pub source_id: SourceId,
}
