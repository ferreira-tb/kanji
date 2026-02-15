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
use std::sync::{Arc, Mutex};

#[cfg(desktop)]
use {anyhow::Result, std::fs, std::path::Path, std::sync::MutexGuard};

#[cfg(all(desktop, debug_assertions))]
const URL: &str = env!("KANJI_DATABASE_URL_DEBUG");
#[cfg(all(desktop, not(debug_assertions)))]
const URL: &str = env!("KANJI_DATABASE_URL");

#[cfg(all(desktop, debug_assertions))]
const BACKUP_DIR: &str = env!("KANJI_BACKUP_DIR_DEBUG");
#[cfg(all(desktop, not(debug_assertions)))]
const BACKUP_DIR: &str = env!("KANJI_BACKUP_DIR");

#[must_use]
#[derive(Clone)]
pub struct DatabaseHandle(Arc<Mutex<SqliteConnection>>);

#[cfg(desktop)]
impl DatabaseHandle {
  pub fn new() -> Result<Self> {
    if let Some(dir) = Path::new(URL).parent() {
      fs::create_dir_all(dir)?;
    }

    backup::run(false)?;
    let mut conn = SqliteConnection::establish(URL)?;
    migration::run_pending_migrations(&mut conn);
    Ok(Self(Arc::new(Mutex::new(conn))))
  }

  pub fn conn(&self) -> MutexGuard<'_, SqliteConnection> {
    self
      .0
      .lock()
      .expect("connection mutex is poisoned")
  }
}
