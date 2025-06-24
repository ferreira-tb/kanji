use crate::kanji::KanjiChar;
use crate::util::walk_dir;
use anyhow::Result;
use itertools::Itertools;
use memchr::memmem::Finder;
use rand::seq::SliceRandom;
use serde::{Serialize, Serializer};
use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use tauri::async_runtime::spawn_blocking;

static ID: AtomicU64 = AtomicU64::new(0);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
  id: SnippetId,
  content: Arc<str>,
  source: Source,
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
struct Source {
  path: Arc<Path>,
  line: usize,
}

pub async fn search(dir: PathBuf, kanji: KanjiChar) -> Result<Vec<Snippet>> {
  spawn_blocking(move || blocking_search(&dir, kanji)).await?
}

fn blocking_search(dir: &Path, kanji: KanjiChar) -> Result<Vec<Snippet>> {
  let mut snippets = Vec::new();
  let mut buf = [0u8; 4];
  let finder = Finder::new(kanji.encode_utf8(&mut buf));

  for path in walk_dir(dir) {
    let path = Arc::from(path);
    let file = File::open_buffered(&path)?;
    for (line, text) in file.lines().enumerate() {
      let Ok(text) = text else { continue };
      let bytes = text.as_bytes();
      if finder.find(bytes).is_some() {
        let path = Arc::clone(&path);
        snippets.push(Snippet {
          id: SnippetId::next(),
          content: Arc::from(text),
          source: Source { path, line },
        });
      }
    }
  }

  snippets = snippets
    .into_iter()
    .unique_by(|snippet: &Snippet| Arc::clone(&snippet.content))
    .collect();

  let mut rng = rand::rng();
  snippets.shuffle(&mut rng);

  snippets = snippets.into_iter().take(1000).collect();

  Ok(snippets)
}
