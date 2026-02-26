use anyhow::Result;
use std::env;
use std::path::PathBuf;
use tauri::Wry;
use tauri::path::PathResolver;

#[cfg(desktop)]
use {
  crate::database::DatabaseHandle,
  crate::server::Server,
  tauri::{Manager, State},
};

#[cfg(desktop)]
pub trait ManagerExt: Manager<Wry> {
  fn database(&self) -> State<'_, DatabaseHandle> {
    self.app_handle().state::<DatabaseHandle>()
  }

  fn server(&self) -> State<'_, Server> {
    self.app_handle().state::<Server>()
  }
}

#[cfg(desktop)]
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
      && let Some(home) = env::home_dir()
    {
      home.join(".tsukilabs/kanji")
    } else {
      self.app_data_dir()?
    };

    if cfg!(debug_assertions) {
      dir.push(".dev");
    }

    Ok(dir)
  }
}
