use crate::database::sql_types::{KanjiChar, SourceId};
use crate::kanji::is_kanji;
use crate::manager::ManagerExt;
use crate::settings::Settings;
use anyhow::Result;
use itertools::Itertools;
use memchr::memmem::Finder;
use rand::seq::SliceRandom;
use serde::{Serialize, Serializer};
use std::fs::File;
use std::io::BufRead;
use std::path::Path as StdPath;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use tauri::AppHandle;
use tauri::async_runtime::spawn_blocking;

static ID: AtomicU64 = AtomicU64::new(0);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
  id: SnippetId,
  content: Arc<str>,
  source: SnippetSource,
}

#[derive(Clone, Copy)]
pub struct SnippetId(u64);

impl SnippetId {
  fn next() -> Self {
    Self(ID.fetch_add(1, Relaxed))
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SnippetSource {
  path: Arc<StdPath>,
  line: usize,
}

pub async fn search(
  app: AppHandle,
  kanji: KanjiChar,
  source: Option<SourceId>,
) -> Result<Vec<Snippet>> {
  spawn_blocking(move || blocking_search(&app, kanji, source)).await?
}

fn blocking_search(
  app: &AppHandle,
  kanji: KanjiChar,
  source: Option<SourceId>,
) -> Result<Vec<Snippet>> {
  let settings = Settings::get(app)?;
  let sources = if let Some(id) = source {
    vec![app.database().get_source(id)?]
  } else {
    app.database().get_sources()?
  };

  let mut snippets = Vec::new();
  let mut buf = [0u8; 4];
  let finder = Finder::new(kanji.encode_utf8(&mut buf));

  for source in sources {
    for path in source.walk() {
      let path = Arc::from(path);
      let file = File::open_buffered(&path)?;
      for (line, text) in file.lines().enumerate() {
        let Ok(text) = text else { continue };
        if !should_skip(&text) && has_min_len(&text, settings.snippet_min_len) {
          let bytes = text.as_bytes();
          if finder.find(bytes).is_some() {
            let path = Arc::clone(&path);
            let line = line.saturating_add(1);
            snippets.push(Snippet {
              id: SnippetId::next(),
              content: Arc::from(text),
              source: SnippetSource { path, line },
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

  if settings.shuffle_snippets {
    let mut rng = rand::rng();
    snippets.shuffle(&mut rng);
  }

  snippets = snippets
    .into_iter()
    .take(settings.snippet_limit)
    .collect();

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
