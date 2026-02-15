pub mod model;
pub mod schema;
pub mod sql_types;

#[cfg(desktop)]
mod backup;
#[cfg(desktop)]
mod r#impl;
#[cfg(desktop)]
mod migration;

use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use std::sync::Arc;
use std::sync::nonpoison::Mutex;

#[cfg(desktop)]
use {
  crate::manager::PathResolverExt,
  anyhow::Result,
  std::fs,
  std::path::Path,
  std::sync::nonpoison::MutexGuard,
  tauri::AppHandle,
  tauri::Manager,
};

#[must_use]
#[derive(Clone)]
pub struct DatabaseHandle(Arc<Mutex<SqliteConnection>>);

#[cfg(desktop)]
impl DatabaseHandle {
  pub fn new(app: &AppHandle) -> Result<Self> {
    let url = app.path().kanji_dir()?.join("kanji.db");
    if let Some(dir) = Path::new(&url).parent() {
      fs::create_dir_all(dir)?;
    }

    backup::run(app, &url, false)?;

    let url = url.to_str().unwrap();
    let mut conn = SqliteConnection::establish(url)?;
    migration::run_pending_migrations(&mut conn);
    Ok(Self(Arc::new(Mutex::new(conn))))
  }

  pub fn conn(&self) -> MutexGuard<'_, SqliteConnection> {
    self.0.lock()
  }
}
