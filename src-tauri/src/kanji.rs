use anyhow::Result;
use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};
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

#[derive(Serialize, Type)]
pub struct Frequency {
  kanji: Kanji,
  seen: u32,
  ratio: f64,
  level: Level,
  sources: Vec<Source>,
}

impl Frequency {
  fn new(kanji: Kanji) -> Frequency {
    Frequency {
      kanji,
      seen: 0,
      ratio: 0.0,
      level: Level::Unknown,
      sources: Vec::default(),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Type)]
struct Kanji {
  character: char,
}

impl Kanji {
  #[allow(clippy::unnecessary_lazy_evaluations)]
  fn from_char(c: char) -> Option<Self> {
    is_kanji(c).then(|| Self { character: c })
  }
}

#[derive(Serialize, Type)]
struct Source {
  name: String,
  seen: u32,
}

#[derive(Serialize, Type)]
#[serde(rename_all = "kebab-case")]
enum Level {
  Common,
  Uncommon,
  Rare,
  VeryRare,
  Unknown,
}

impl Level {
  const fn from_ratio(mut ratio: f64) -> Self {
    ratio *= 100.0;
    if ratio >= 0.1 {
      Self::Common
    } else if ratio >= 0.01 {
      Self::Uncommon
    } else if ratio >= 0.001 {
      Self::Rare
    } else if ratio >= 0.0001 {
      Self::VeryRare
    } else {
      Self::Unknown
    }
  }
}

pub async fn search(path: PathBuf) -> Result<Vec<Frequency>> {
  spawn_blocking(move || blocking_search(path)).await?
}

fn blocking_search(path: PathBuf) -> Result<Vec<Frequency>> {
  let mut map: HashMap<Kanji, Frequency> = HashMap::new();
  for entry in WalkDir::new(path).into_iter().flatten() {
    let path = entry.into_path();
    if path.is_file() && GLOBSET.is_match(&path) {
      for kanji in fs::read_to_string(&path)?
        .chars()
        .filter_map(Kanji::from_char)
      {
        let frequency = map
          .entry(kanji)
          .or_insert_with(|| Frequency::new(kanji));

        frequency.seen = frequency.seen.saturating_add(1);

        if let Some(parent) = path.parent()
          && let Some(name) = parent.file_name()
          && let Some(name) = name.to_str()
        {
          if let Some(source) = frequency
            .sources
            .iter_mut()
            .find(|s| s.name == name)
          {
            source.seen = source.seen.saturating_add(1);
          } else {
            frequency
              .sources
              .push(Source { name: name.to_owned(), seen: 1 });
          }
        }
      }
    }
  }

  let total = map
    .values()
    .map(|frequency| u64::from(frequency.seen))
    .fold(0u64, u64::saturating_add) as f64;

  if total.is_normal() {
    for frequency in map.values_mut() {
      frequency.ratio = f64::from(frequency.seen) / total;
      frequency.level = Level::from_ratio(frequency.ratio);
    }
  }

  Ok(map.into_values().collect())
}

const fn is_kanji(c: char) -> bool {
  // http://www.rikai.com/library/kanjitables/kanji_codes.unicode.shtml
  matches!(c, '\u{4e00}'..='\u{9faf}' | '\u{3400}' ..= '\u{4dbf}')
}

fn glob(glob: &str) -> Glob {
  GlobBuilder::new(glob)
    .case_insensitive(true)
    .build()
    .unwrap()
}
