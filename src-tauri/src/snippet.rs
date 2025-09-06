use crate::database::model::source::Source;
use crate::database::sql_types::{KanjiChar, SourceId, SourceWeight};
use crate::kanji::is_kanji;
use crate::manager::ManagerExt;
use crate::settings::Settings;
use anyhow::Result;
use itertools::Itertools;
use memchr::memmem::Finder;
use rand::seq::{IndexedRandom, SliceRandom};
use serde::{Serialize, Serializer};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::path::Path as StdPath;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use tauri::AppHandle;
use tauri::async_runtime::spawn_blocking;

static ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
  id: SnippetId,
  content: Arc<str>,
  source: SnippetSource,
}

impl Snippet {
  pub fn content(&self) -> &str {
    &self.content
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SnippetId(u64);

impl SnippetId {
  fn new() -> Self {
    Self(ID.fetch_add(1, Relaxed))
  }
}

impl Default for SnippetId {
  fn default() -> Self {
    Self::new()
  }
}

impl Serialize for SnippetId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.0.to_string())
  }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SnippetSource {
  name: Arc<str>,
  path: Arc<StdPath>,
  weight: SourceWeight,
  line: usize,
}

pub async fn search(
  app: AppHandle,
  kanji: KanjiChar,
  source: Option<SourceId>,
) -> Result<Vec<Snippet>> {
  spawn_blocking(move || blocking_search(&app, kanji, source)).await?
}

pub fn blocking_search(
  app: &AppHandle,
  kanji: KanjiChar,
  source: Option<SourceId>,
) -> Result<Vec<Snippet>> {
  let settings = Settings::get(app)?;
  let sources = if let Some(id) = source {
    vec![app.database().get_source(id)?]
  } else {
    app.database().get_enabled_sources()?
  };

  blocking_search_with_options(kanji)
    .sources(&sources)
    .min_len(settings.snippet_min_len)
    .limit(settings.snippet_limit)
    .shuffle(settings.shuffle_snippets)
    .call()
}

#[bon::builder]
pub fn blocking_search_with_options(
  #[builder(start_fn)] kanji: KanjiChar,
  #[builder(default)] sources: &[Source],
  #[builder(default = 5)] min_len: usize,
  #[builder(default = 1000)] limit: usize,
  #[builder(default = true)] shuffle: bool,
) -> Result<Vec<Snippet>> {
  let mut snippets = Vec::new();
  let mut buf = [0u8; 4];
  let finder = Finder::new(kanji.encode_utf8(&mut buf));

  for source in sources {
    let name = Arc::from(source.name.as_str());
    let weight = source.weight;

    for path in source.walk() {
      let path = Arc::from(path);
      let file = File::open_buffered(&path)?;

      for (line, text) in file.lines().enumerate() {
        let Ok(text) = text else { continue };
        if !should_skip(&text) && has_min_len(&text, min_len) {
          let bytes = text.as_bytes();
          if finder.find(bytes).is_some() {
            let name = Arc::clone(&name);
            let path = Arc::clone(&path);
            let line = line.saturating_add(1);

            snippets.push(Snippet {
              id: SnippetId::new(),
              content: Arc::from(text),
              source: SnippetSource { name, path, weight, line },
            });
          }
        }
      }
    }
  }

  snippets = snippets
    .into_iter()
    .unique_by(|snippet: &Snippet| Arc::clone(&snippet.content))
    .collect();

  let mut rng = rand::rng();
  let chosen = snippets
    .iter()
    .map(|snippet| (snippet.id, snippet.source.weight))
    .collect_vec()
    .choose_multiple_weighted(&mut rng, limit, |(_, weight)| *weight)?
    .map(|(id, _)| *id)
    .collect::<HashSet<_>>();

  snippets.retain(|snippet| chosen.contains(&snippet.id));

  if shuffle {
    snippets.shuffle(&mut rng);
  }

  Ok(snippets)
}

fn should_skip(text: &str) -> bool {
  let text = text.trim_start();
  text.starts_with('#') || text.starts_with('<')
}

fn has_min_len(text: &str, min_len: usize) -> bool {
  let mut matches: usize = 0;
  for char in text.chars() {
    if is_kanji(char) {
      matches = matches.saturating_add(1);
    }

    if matches >= min_len {
      return true;
    }
  }

  false
}
