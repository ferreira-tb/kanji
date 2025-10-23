use crate::database::sql_types::{Path, SourceId, SourceWeight, Zoned};
use anyhow::Result;
use bon::Builder;
use diesel::prelude::*;
use serde::Serialize;
use std::path::PathBuf;
use tauri::AppHandle;

#[cfg(desktop)]
use {
  crate::manager::ManagerExt,
  globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder},
  std::sync::LazyLock,
  walkdir::{DirEntry, WalkDir},
};

#[cfg(desktop)]
static GLOBSET: LazyLock<GlobSet> = LazyLock::new(|| {
  GlobSetBuilder::new()
    .add(glob("*.md"))
    .add(glob("*.txt"))
    .build()
    .unwrap()
});

#[derive(Identifiable, Queryable, Selectable, Clone, Debug, Serialize)]
#[diesel(table_name = crate::database::schema::source)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Source {
  pub id: SourceId,
  pub path: Path,
  pub name: String,
  pub created_at: Zoned,
  pub updated_at: Zoned,
  pub enabled: bool,
  pub weight: SourceWeight,
}

#[cfg(desktop)]
impl Source {
  pub fn walk(&self) -> Vec<PathBuf> {
    WalkDir::new(&*self.path)
      .max_depth(1)
      .sort_by_file_name()
      .into_iter()
      .flatten()
      .map(DirEntry::into_path)
      .filter(|path| path.is_file() && GLOBSET.is_match(path))
      .collect()
  }
}

#[derive(Insertable, Builder, Clone, Debug)]
#[diesel(table_name = crate::database::schema::source)]
pub struct NewSource<'a> {
  #[builder(start_fn, into)]
  path: Path,

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

#[cfg(desktop)]
fn glob(glob: &str) -> Glob {
  GlobBuilder::new(glob)
    .case_insensitive(true)
    .build()
    .unwrap()
}
