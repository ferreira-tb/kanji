use crate::database::sql_types::{BookmarkId, SourceId, Zoned};
use crate::snippet::Snippet;
use bon::Builder;
use diesel::prelude::*;
use serde::Serialize;

#[cfg(desktop)]
use {crate::manager::ManagerExt, anyhow::Result, tauri::AppHandle};

#[derive(Identifiable, Queryable, Selectable, Clone, Debug, Serialize)]
#[diesel(table_name = crate::database::schema::bookmark)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
  pub id: BookmarkId,
  pub snippet: String,
  pub source_id: SourceId,
  pub created_at: Zoned,
}

#[derive(Insertable, Builder, Clone, Debug)]
#[diesel(table_name = crate::database::schema::bookmark)]
pub struct NewBookmark {
  snippet: String,
  source_id: SourceId,

  #[builder(skip = Zoned::now())]
  created_at: Zoned,
}

#[cfg(desktop)]
impl NewBookmark {
  pub fn create(self, app: &AppHandle) -> Result<BookmarkId> {
    app.database().create_bookmark(&self)
  }
}

impl From<&Snippet> for NewBookmark {
  fn from(snippet: &Snippet) -> Self {
    Self::builder()
      .snippet(snippet.content().to_owned())
      .source_id(snippet.source().id())
      .build()
  }
}
