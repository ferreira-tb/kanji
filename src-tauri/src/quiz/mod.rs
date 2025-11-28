mod source_stats;

#[cfg(desktop)]
mod chunk;
#[cfg(desktop)]
mod html;

pub use source_stats::QuizSourceStats;

use crate::database::sql_types::{KanjiChar, SourceGroupId, SourceId};
use crate::snippet::Snippet;
use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(desktop)]
use {
  crate::kanji::blocking_search_with_options,
  crate::manager::ManagerExt,
  crate::settings,
  anyhow::{Error, Result, bail},
  itertools::Itertools,
  rand::seq::{IndexedRandom, IteratorRandom},
  tauri::AppHandle,
  tauri::async_runtime::spawn_blocking,
};

#[cfg(desktop)]
const MARUMARU: &str = "◯";

#[derive(Serialize)]
pub struct Quiz(Vec<QuizQuestion>);

#[cfg(desktop)]
impl Quiz {
  pub async fn new(app: AppHandle, kind: QuizKind) -> Result<Self> {
    match kind {
      QuizKind::Chunk { chunk } => Self::from_chunk(app, chunk).await,
      QuizKind::RandomChunk => Self::from_random_chunk(app).await,
      QuizKind::Source { ids } => Self::from_sources(app, ids).await,
      QuizKind::RandomSource => Self::from_random_source(app).await,
      QuizKind::SourceGroup { ids } => Self::from_source_groups(app, ids).await,
      QuizKind::RandomSourceGroup => Self::from_random_source_group(app).await,
      QuizKind::Url { urls } => html::from_urls(app, urls).await,
    }
  }

  async fn from_chunk(app: AppHandle, kanjis: Vec<KanjiChar>) -> Result<Self> {
    let sources = app.database().get_enabled_sources()?;
    chunk::with_sources(app, kanjis, sources).await
  }

  async fn from_random_chunk(app: AppHandle) -> Result<Self> {
    let chunk_size = settings::set_chunk_size(&app);
    let kanjis = app
      .database()
      .get_kanji_chars()?
      .choose_multiple(&mut rand::rng(), chunk_size)
      .copied()
      .collect_vec();

    Self::from_chunk(app, kanjis).await
  }

  async fn from_sources(app: AppHandle, ids: Vec<SourceId>) -> Result<Self> {
    let sources = app.database().get_sources_by(&ids)?;
    let stats = spawn_blocking({
      let app = app.clone();
      let sources = sources.clone();
      move || {
        blocking_search_with_options(&app)
          .sources(&sources)
          .call()
      }
    });

    let chunk_size = settings::set_chunk_size(&app);
    let kanjis = stats
      .await??
      .into_iter()
      .map(|stat| stat.character())
      .choose_multiple(&mut rand::rng(), chunk_size);

    chunk::with_sources(app, kanjis, sources).await
  }

  async fn from_random_source(app: AppHandle) -> Result<Self> {
    let Some(id) = app
      .database()
      .get_source_ids()?
      .choose(&mut rand::rng())
      .copied()
    else {
      bail!("No source found");
    };

    Self::from_sources(app, vec![id]).await
  }

  async fn from_source_groups(app: AppHandle, ids: Vec<SourceGroupId>) -> Result<Self> {
    let sources = spawn_blocking({
      let app = app.clone();
      move || {
        let db = app.database();
        let sources = ids
          .into_iter()
          .map(|id| db.get_source_group_source_ids(id))
          .try_collect::<_, Vec<Vec<SourceId>>, _>()?
          .into_iter()
          .flatten()
          .unique()
          .collect_vec();

        Ok::<_, Error>(sources)
      }
    });

    Self::from_sources(app, sources.await??).await
  }

  async fn from_random_source_group(app: AppHandle) -> Result<Self> {
    let Some(id) = app
      .database()
      .get_source_group_ids()?
      .choose(&mut rand::rng())
      .copied()
    else {
      bail!("No source group found");
    };

    Self::from_source_groups(app, vec![id]).await
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum QuizKind {
  Chunk { chunk: Vec<KanjiChar> },
  RandomChunk,
  Source { ids: Vec<SourceId> },
  RandomSource,
  SourceGroup { ids: Vec<SourceGroupId> },
  RandomSourceGroup,
  Url { urls: Vec<Url> },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizQuestion {
  snippet: Snippet,
  censored: String,
  answer: KanjiChar,
  options: Vec<KanjiChar>,
}
