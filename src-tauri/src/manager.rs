use crate::database::DatabaseHandle;
use crate::server::Server;
use anyhow::Result;
use std::env::home_dir;
use std::path::PathBuf;
use tauri::path::PathResolver;
use tauri::{Manager, State, Wry};

pub trait ManagerExt: Manager<Wry> {
  fn database(&self) -> State<'_, DatabaseHandle> {
    self.app_handle().state::<DatabaseHandle>()
  }

  fn server(&self) -> State<'_, Server> {
    self.app_handle().state::<Server>()
  }
}

impl<T: Manager<Wry>> ManagerExt for T {}

pub trait PathResolverExt {
  fn kanji_dir(&self) -> Result<PathBuf>;

  fn backup_dir(&self) -> Result<PathBuf> {
    Ok(self.kanji_dir()?.join("backup"))
  }
}

impl PathResolverExt for PathResolver<Wry> {
  fn kanji_dir(&self) -> Result<PathBuf> {
    let mut dir = if cfg!(desktop)
      && let Some(home) = home_dir()
    {
      home
        .join(".tsukilabs")
        .join(env!("CARGO_PKG_NAME"))
    } else {
      self
        .app_cache_dir()
        .or_else(|_| self.app_data_dir())?
    };

    if cfg!(debug_assertions) {
      dir.push("dev");
    }

    Ok(dir)
  }
}
