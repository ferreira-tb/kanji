use crate::error::WrapOk;
use anyhow::{Error, Result};
use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};
use itertools::Itertools;
use serde::Serialize;
use specta::Type;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;
use tauri::async_runtime::spawn_blocking;
use walkdir::WalkDir;

static GLOBSET: LazyLock<GlobSet> = LazyLock::new(|| {
  GlobSetBuilder::new()
    .add(glob("*.md"))
    .add(glob("*.txt"))
    .build()
    .unwrap()
});

#[derive(Clone, Copy, Serialize, Type)]
pub struct Frequency {
  kanji: Kanji,
  amount: u32,
}

impl Frequency {
  fn new(kanji: Kanji, amount: u32) -> Self {
    Self { kanji, amount }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Type)]
pub struct Kanji {
  character: char,
}

impl Kanji {
  #[allow(clippy::unnecessary_lazy_evaluations)]
  pub fn from_char(c: char) -> Option<Self> {
    is_kanji(c).then(|| Self { character: c })
  }
}

pub async fn search(path: PathBuf) -> Result<Vec<Frequency>> {
  let handle = spawn_blocking(move || {
    let mut frequency: HashMap<Kanji, u32> = HashMap::new();
    for entry in WalkDir::new(path).into_iter().flatten() {
      let entry = entry.into_path();
      if entry.is_file() && GLOBSET.is_match(&entry) {
        for kanji in fs::read_to_string(&entry)?
          .chars()
          .filter_map(Kanji::from_char)
        {
          frequency
            .entry(kanji)
            .and_modify(|e| *e = e.saturating_add(1))
            .or_insert(1);
        }
      }
    }

    Ok::<_, Error>(frequency)
  });

  handle
    .await??
    .into_iter()
    .map(|(kanji, amount)| Frequency::new(kanji, amount))
    .collect_vec()
    .wrap_ok()
}

pub const fn is_kanji(c: char) -> bool {
  // http://www.rikai.com/library/kanjitables/kanji_codes.unicode.shtml
  matches!(c, '\u{4e00}'..='\u{9faf}' | '\u{3400}' ..= '\u{4dbf}')
}

fn glob(glob: &str) -> Glob {
  GlobBuilder::new(glob)
    .case_insensitive(true)
    .build()
    .unwrap()
}
