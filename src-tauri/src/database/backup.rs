use crate::manager::PathResolverExt;
use anyhow::Result;
use jiff::Zoned;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager};

#[derive(Debug, Default, Serialize, Deserialize)]
struct Backup {
  date: Option<Zoned>,
  version: Option<Version>,
}

impl Backup {
  const INTERVAL: i32 = 15;

  fn read(path: &Path) -> Self {
    let backup = fs::read(path).unwrap_or_default();
    serde_json::from_slice(&backup).unwrap_or_default()
  }

  fn write(self, path: &Path) -> Result<()> {
    let backup = serde_json::to_vec_pretty(&self)?;
    fs::write(path, backup)?;
    Ok(())
  }

  fn should_backup(&self) -> bool {
    self.days_since() >= Backup::INTERVAL || self.is_outdated()
  }

  fn days_since(&self) -> i32 {
    if let Some(date) = self.date.as_ref()
      && let Ok(span) = date.until(&Zoned::now())
    {
      span.get_days()
    } else {
      i32::MAX
    }
  }

  fn is_outdated(&self) -> bool {
    self
      .version
      .as_ref()
      .is_none_or(|it| *it < version())
  }
}

pub fn run(app: &AppHandle, url: &Path, force: bool) -> Result<()> {
  if !fs::exists(url)? {
    return Ok(());
  }

  let dir = app.path().backup_dir()?;
  fs::create_dir_all(&dir)?;

  let backup_file = dir.join("backup.json");
  let mut backup = Backup::read(&backup_file);

  if force || backup.should_backup() {
    let version = version();
    let now = Zoned::now().strftime("%Y%m%d%H%M%S");
    let database = dir.join(format!("kanji-{version}.{now}.db"));

    fs::copy(url, &database)?;

    backup.date = Some(Zoned::now());
    backup.version = Some(version);
    backup.write(&backup_file)?;
  }

  Ok(())
}

fn version() -> Version {
  Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}
