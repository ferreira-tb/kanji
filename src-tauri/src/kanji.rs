use crate::database::sql_types::{KanjiChar, SourceId};
use serde::Serialize;
use std::sync::Arc;

#[cfg(desktop)]
use {
  crate::database::model::kanji::NewKanji,
  crate::database::model::source::Source,
  crate::manager::ManagerExt,
  anyhow::Result,
  itertools::Itertools,
  std::collections::HashMap,
  std::fs,
  std::sync::atomic::AtomicBool,
  std::sync::atomic::Ordering::Relaxed,
  tauri::AppHandle,
  tauri::async_runtime::spawn_blocking,
};

#[cfg(desktop)]
static IS_FIRST_SEARCH: AtomicBool = AtomicBool::new(true);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KanjiStats {
  character: KanjiChar,
  seen: u32,
  ratio: f64,
  level: Level,
  sources: Vec<KanjiStatsSource>,
  quizzes: u64,
  correct_quiz_answers: u64,
  quiz_accuracy: f64,
}

impl KanjiStats {
  fn new(kanji: KanjiChar) -> KanjiStats {
    KanjiStats {
      character: kanji,
      seen: 0,
      ratio: 0.0,
      level: Level::Unknown,
      sources: Vec::default(),
      quizzes: 0,
      correct_quiz_answers: 0,
      quiz_accuracy: 0.0,
    }
  }

  pub fn character(&self) -> KanjiChar {
    self.character
  }

  pub fn seen(&self) -> u32 {
    self.seen
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct KanjiStatsSource {
  id: SourceId,
  name: Arc<str>,
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

#[cfg(desktop)]
pub async fn search(app: AppHandle) -> Result<Vec<KanjiStats>> {
  spawn_blocking(move || blocking_search(&app)).await?
}

#[cfg(desktop)]
fn blocking_search(app: &AppHandle) -> Result<Vec<KanjiStats>> {
  let sources = app.database().get_enabled_sources()?;
  blocking_search_with_options(app)
    .sources(&sources)
    .call()
}

#[cfg(desktop)]
#[bon::builder]
pub fn blocking_search_with_options(
  #[builder(start_fn)] app: &AppHandle,
  #[builder(default)] sources: &[Source],
) -> Result<Vec<KanjiStats>> {
  let database = app.database();
  let mut kanjis: HashMap<KanjiChar, KanjiStats> = HashMap::new();

  for source in sources {
    let id = source.id;
    let name: Arc<str> = Arc::from(source.name.as_str());
    for file in source.walk() {
      for character in fs::read_to_string(&file)?
        .chars()
        .filter_map(KanjiChar::from_char)
      {
        let kanji = kanjis
          .entry(character)
          .or_insert_with(|| KanjiStats::new(character));

        kanji.seen = kanji.seen.saturating_add(1);

        if let Some(source) = kanji
          .sources
          .iter_mut()
          .find(|it| it.name == name)
        {
          source.seen = source.seen.saturating_add(1);
        } else {
          let name = Arc::clone(&name);
          kanji
            .sources
            .push(KanjiStatsSource { id, name, seen: 1 });
        }
      }
    }
  }

  let total = kanjis
    .values()
    .map(|kanji| u64::from(kanji.seen))
    .fold(0u64, u64::saturating_add) as f64;

  for kanji in kanjis.values_mut() {
    if total.is_normal() {
      kanji.ratio = f64::from(kanji.seen) / total;
      kanji.level = Level::from_ratio(kanji.ratio);
    }

    kanji.quizzes = database.count_quizzes(kanji.character)?;

    if kanji.quizzes > 0 {
      kanji.correct_quiz_answers = database.count_correct_quizzes(kanji.character)?;
      kanji.quiz_accuracy = (kanji.correct_quiz_answers as f64) / (kanji.quizzes as f64);
    }
  }

  let kanjis = kanjis.into_values().collect_vec();
  if IS_FIRST_SEARCH.load(Relaxed) {
    IS_FIRST_SEARCH.store(false, Relaxed);
    for kanji in &kanjis {
      if !database.has_kanji(kanji.character)? {
        NewKanji::builder(kanji.character)
          .build()
          .create(app)?;
      }
    }
  }

  Ok(kanjis)
}

pub const fn is_kanji(c: char) -> bool {
  // http://www.rikai.com/library/kanjitables/kanji_codes.unicode.shtml
  matches!(c, '\u{4e00}'..='\u{9faf}' | '\u{3400}' ..= '\u{4dbf}')
}
