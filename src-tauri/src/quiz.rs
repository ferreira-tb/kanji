use crate::database::sql_types::KanjiChar;
use crate::settings::Settings;
use crate::snippet::{self, Snippet};
use serde::{Deserialize, Serialize};

#[cfg(desktop)]
use {
  crate::database::model::source::Source,
  crate::database::sql_types::SourceId,
  crate::kanji::blocking_search_with_options,
  crate::manager::ManagerExt,
  anyhow::{Result, bail},
  itertools::Itertools,
  rand::seq::{IndexedRandom, IteratorRandom, SliceRandom},
  std::sync::Arc,
  tauri::AppHandle,
  tauri::async_runtime::spawn_blocking,
  tokio::sync::{Mutex, Semaphore},
  tokio::task::JoinSet,
};

const MARUMARU: &str = "◯";

#[derive(Serialize)]
pub struct Quiz(Vec<QuizQuestion>);

#[cfg(desktop)]
impl Quiz {
  pub async fn new(app: AppHandle, kind: QuizKind) -> Result<Self> {
    match kind {
      QuizKind::Chunk { chunk } => Self::from_chunk(app, chunk).await,
      QuizKind::RandomChunk => Self::from_random_chunk(app).await,
      QuizKind::Source { id } => Self::from_source(app, id).await,
      QuizKind::RandomSource => Self::from_random_source(app).await,
    }
  }

  async fn from_chunk(app: AppHandle, kanjis: Vec<KanjiChar>) -> Result<Self> {
    let sources = app.database().get_enabled_sources()?;
    Self::from_chunk_with_sources(app, kanjis, sources).await
  }

  async fn from_chunk_with_sources(
    app: AppHandle,
    kanjis: Vec<KanjiChar>,
    sources: Vec<Source>,
  ) -> Result<Self> {
    let db = app.database();
    let chars = Arc::from(db.get_kanji_chars()?);
    let sources = Arc::from(sources);
    let questions = Arc::new(Mutex::new(Vec::new()));
    let semaphore = Arc::new(Semaphore::new(100));
    let settings = Arc::new(Settings::get(&app)?);

    let mut set: JoinSet<Result<()>> = kanjis
      .into_iter()
      .map(|kanji| {
        let app = app.clone();
        let chars = Arc::clone(&chars);
        let sources = Arc::clone(&sources);
        let questions = Arc::clone(&questions);
        let semaphore = Arc::clone(&semaphore);
        let settings = Arc::clone(&settings);

        async move {
          let permit = semaphore.acquire().await?;
          let snippet = spawn_blocking(move || {
            snippet::blocking_search_with_options(&app, kanji)
              .sources(&sources)
              .limit(1)
              .min_len(settings.snippet_min_len)
              .threshold(settings.snippet_kanji_threshold)
              .shuffle(true)
              .call()
          });

          if let Some(snippet) = snippet.await??.pop() {
            let censored = snippet.content().replace(*kanji, MARUMARU);
            let mut questions = questions.lock().await;
            questions.push(QuizQuestion {
              snippet,
              censored,
              answer: kanji,
              options: pick_options(kanji, &chars),
            });
          }

          drop(permit);

          Ok(())
        }
      })
      .collect();

    while let Some(result) = set.join_next().await {
      result??;
    }

    let mut questions = Arc::try_unwrap(questions)
      .expect("should be the only strong ref")
      .into_inner();

    let mut rng = &mut rand::rng();
    questions.shuffle(&mut rng);

    Ok(Self(questions))
  }

  async fn from_random_chunk(app: AppHandle) -> Result<Self> {
    let chunk_size = Settings::get(&app)?.set_chunk_size;
    let kanjis = app
      .database()
      .get_kanji_chars()?
      .choose_multiple(&mut rand::rng(), chunk_size)
      .copied()
      .collect_vec();

    Self::from_chunk(app, kanjis).await
  }

  async fn from_source(app: AppHandle, id: SourceId) -> Result<Self> {
    let source = app.database().get_source(id)?;
    let stats = spawn_blocking({
      let app = app.clone();
      let source = source.clone();
      move || {
        blocking_search_with_options(&app)
          .sources(&[source])
          .call()
      }
    });

    let chunk_size = Settings::get(&app)?.set_chunk_size;
    let kanjis = stats
      .await??
      .into_iter()
      .map(|stat| stat.character())
      .choose_multiple(&mut rand::rng(), chunk_size);

    Self::from_chunk_with_sources(app, kanjis, vec![source]).await
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

    Self::from_source(app, id).await
  }
}

#[cfg(desktop)]
fn pick_options(answer: KanjiChar, pool: &[KanjiChar]) -> Vec<KanjiChar> {
  let mut rng = &mut rand::rng();
  let mut options = pool
    .iter()
    .filter(|kanji| **kanji != answer)
    .copied()
    .choose_multiple(&mut rng, 9);

  options.push(answer);
  options.shuffle(&mut rng);
  options
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum QuizKind {
  Chunk { chunk: Vec<KanjiChar> },
  RandomChunk,
  Source { id: SourceId },
  RandomSource,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizQuestion {
  snippet: Snippet,
  censored: String,
  answer: KanjiChar,
  options: Vec<KanjiChar>,
}
