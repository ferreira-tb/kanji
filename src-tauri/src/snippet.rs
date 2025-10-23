use crate::database::sql_types::{BookmarkId, KanjiChar, SourceId, SourceWeight};
use crate::settings::Settings;
use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashSet;
use std::path::Path as StdPath;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use tauri::AppHandle;

#[cfg(desktop)]
use {
  crate::database::model::bookmark::NewBookmark,
  crate::database::model::source::Source,
  crate::kanji::is_kanji,
  crate::manager::ManagerExt,
  itertools::Itertools,
  memchr::memmem::Finder,
  rand::seq::{IndexedRandom, SliceRandom},
  std::fs::File,
  std::io::BufRead,
  tauri::async_runtime::spawn_blocking,
};

static ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
  id: SnippetId,
  content: Arc<str>,
  source: SnippetSource,

  #[serde(skip_deserializing)]
  bookmark: Option<BookmarkId>,
}

impl Snippet {
  pub fn content(&self) -> &str {
    &self.content
  }

  pub fn source(&self) -> &SnippetSource {
    &self.source
  }

  #[cfg(desktop)]
  pub fn create_bookmark(&self, app: &AppHandle) -> Result<BookmarkId> {
    NewBookmark::from(self).create(app)
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

impl<'de> Deserialize<'de> for SnippetId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error as _;
    String::deserialize(deserializer)?
      .parse::<u64>()
      .map(Self)
      .map_err(D::Error::custom)
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetSource {
  id: SourceId,
  name: Arc<str>,
  path: Arc<StdPath>,
  weight: SourceWeight,
  line: usize,
}

impl SnippetSource {
  pub fn id(&self) -> SourceId {
    self.id
  }
}

#[cfg(desktop)]
pub async fn search(
  app: AppHandle,
  kanji: KanjiChar,
  source: Option<SourceId>,
) -> Result<Vec<Snippet>> {
  spawn_blocking(move || blocking_search(&app, kanji, source)).await?
}

#[cfg(desktop)]
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

  blocking_search_with_options(app, kanji)
    .sources(&sources)
    .limit(settings.snippet_limit)
    .min_len(settings.snippet_min_len)
    .threshold(settings.snippet_kanji_threshold)
    .shuffle(settings.shuffle_snippets)
    .call()
}

#[cfg(desktop)]
#[bon::builder]
pub fn blocking_search_with_options(
  #[builder(start_fn)] app: &AppHandle,
  #[builder(start_fn)] kanji: KanjiChar,
  #[builder(default)] sources: &[Source],
  #[builder(default = Settings::DEFAULT_SNIPPET_LIMIT)] limit: usize,
  #[builder(default = Settings::DEFAULT_SNIPPET_MIN_LEN)] min_len: usize,
  #[builder(default = Settings::DEFAULT_SNIPPET_KANJI_THRESHOLD)] threshold: f64,
  #[builder(default = Settings::DEFAULT_SHUFFLE_SNIPPETS)] shuffle: bool,
) -> Result<Vec<Snippet>> {
  let mut snippets = Vec::new();
  let db = app.database();

  let mut buf = [0u8; 4];
  let finder = Finder::new(kanji.encode_utf8(&mut buf));

  for source in sources {
    let source_id = source.id;
    let name = Arc::from(source.name.as_str());
    let weight = source.weight;

    for path in source.walk() {
      let path = Arc::from(path);
      let file = File::open_buffered(&path)?;

      for (line, text) in file.lines().enumerate() {
        let Ok(text) = text else { continue };

        let text = text.trim();
        if !should_skip(text, min_len, threshold) {
          let bytes = text.as_bytes();
          if finder.find(bytes).is_some() {
            let name = Arc::clone(&name);
            let path = Arc::clone(&path);
            let line = line.saturating_add(1);
            let source = SnippetSource {
              id: source_id,
              name,
              path,
              weight,
              line,
            };

            snippets.push(Snippet {
              id: SnippetId::new(),
              content: Arc::from(text),
              source,
              bookmark: db.get_bookmark_id(text)?,
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

#[cfg(desktop)]
fn should_skip(text: &str, min_len: usize, threshold: f64) -> bool {
  if text.is_empty() || text.starts_with('#') || text.starts_with('<') {
    return true;
  }

  let mut chars: u32 = 0;
  let mut matches: u32 = 0;
  for char in text.chars() {
    chars = chars.saturating_add(1);
    if is_kanji(char) {
      matches = matches.saturating_add(1);
    }
  }

  if (matches as usize) < min_len {
    true
  } else {
    let chars = f64::from(chars);
    let matches = f64::from(matches);
    chars < 1.0 || (matches / chars) < threshold
  }
}
