use crate::util::walk_dir;
use anyhow::Result;
use derive_more::{Deref, Display};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::async_runtime::spawn_blocking;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Kanji {
  character: KanjiChar,
  seen: u32,
  ratio: f64,
  level: Level,
  sources: Vec<Source>,
}

impl Kanji {
  fn new(kanji: KanjiChar) -> Kanji {
    Kanji {
      character: kanji,
      seen: 0,
      ratio: 0.0,
      level: Level::Unknown,
      sources: Vec::default(),
    }
  }

  pub fn character(&self) -> KanjiChar {
    self.character
  }

  pub fn seen(&self) -> u32 {
    self.seen
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deref, Display, Deserialize, Serialize)]
pub struct KanjiChar(char);

impl KanjiChar {
  fn from_char(c: char) -> Option<Self> {
    is_kanji(c).then_some(Self(c))
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Source {
  name: String,
  seen: u32,
}

#[derive(Serialize)]
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

pub async fn search(dir: PathBuf) -> Result<Vec<Kanji>> {
  spawn_blocking(move || blocking_search(&dir)).await?
}

fn blocking_search(dir: &Path) -> Result<Vec<Kanji>> {
  let mut kanjis: HashMap<KanjiChar, Kanji> = HashMap::new();
  for path in walk_dir(dir) {
    for character in fs::read_to_string(&path)?
      .chars()
      .filter_map(KanjiChar::from_char)
    {
      let kanji = kanjis
        .entry(character)
        .or_insert_with(|| Kanji::new(character));

      kanji.seen = kanji.seen.saturating_add(1);

      if let Some(parent) = path.parent()
        && let Some(name) = parent.file_name()
        && let Some(name) = name.to_str()
      {
        if let Some(source) = kanji
          .sources
          .iter_mut()
          .find(|s| s.name == name)
        {
          source.seen = source.seen.saturating_add(1);
        } else {
          kanji
            .sources
            .push(Source { name: name.to_owned(), seen: 1 });
        }
      }
    }
  }

  let total = kanjis
    .values()
    .map(|kanji| u64::from(kanji.seen))
    .fold(0u64, u64::saturating_add) as f64;

  if total.is_normal() {
    for kanji in kanjis.values_mut() {
      kanji.ratio = f64::from(kanji.seen) / total;
      kanji.level = Level::from_ratio(kanji.ratio);
    }
  }

  Ok(kanjis.into_values().collect())
}

pub const fn is_kanji(c: char) -> bool {
  // http://www.rikai.com/library/kanjitables/kanji_codes.unicode.shtml
  matches!(c, '\u{4e00}'..='\u{9faf}' | '\u{3400}' ..= '\u{4dbf}')
}
