use super::{BACKUP_DIR, URL};
use anyhow::Result;
use jiff::Zoned;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Backup {
  date: Option<Zoned>,
  version: Option<Version>,
}

impl Backup {
  const INTERVAL: i32 = 15;

  fn read() -> Self {
    let backup = fs::read(path()).unwrap_or_default();
    serde_json::from_slice(&backup).unwrap_or_default()
  }

  fn write(self) -> Result<()> {
    let backup = serde_json::to_vec_pretty(&self)?;
    fs::write(path(), backup)?;
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

pub fn run(force: bool) -> Result<()> {
  let dir = PathBuf::from(BACKUP_DIR);
  fs::create_dir_all(&dir)?;
  let mut backup = Backup::read();

  if force || backup.should_backup() {
    let version = version();
    let now = Zoned::now().strftime("%Y%m%d%H%M%S");
    let path = dir.join(format!("kanji-{version}.{now}.db"));

    fs::copy(URL, path)?;

    backup.date = Some(Zoned::now());
    backup.version = Some(version);
    backup.write()?;
  }

  Ok(())
}

fn path() -> PathBuf {
  PathBuf::from(BACKUP_DIR).join("backup.json")
}

fn version() -> Version {
  Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}
