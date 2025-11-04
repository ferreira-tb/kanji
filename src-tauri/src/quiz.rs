use crate::database::sql_types::{KanjiChar, SourceId};
use crate::snippet::{self, Snippet};
use serde::{Deserialize, Serialize};

#[cfg(desktop)]
use {
  crate::database::model::source::Source,
  crate::kanji::blocking_search_with_options,
  crate::manager::ManagerExt,
  crate::settings::Settings,
  anyhow::{Result, bail},
  itertools::Itertools,
  rand::seq::{IndexedRandom, IteratorRandom, SliceRandom},
  std::sync::Arc,
  tauri::AppHandle,
  tauri::async_runtime::spawn_blocking,
  tokio::sync::{Mutex, Semaphore},
  tokio::task::JoinSet,
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

    let chunk_size = Settings::get(&app)?.set_chunk_size;
    let kanjis = stats
      .await??
      .into_iter()
      .map(|stat| stat.character())
      .choose_multiple(&mut rand::rng(), chunk_size);

    Self::from_chunk_with_sources(app, kanjis, sources).await
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
  Source { ids: Vec<SourceId> },
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizSourceStats {
  source: SourceId,
  quizzes: u64,
  correct_quiz_answers: u64,
  quiz_accuracy: f64,
}

#[cfg(desktop)]
impl QuizSourceStats {
  pub fn new(app: &AppHandle, source: SourceId) -> Result<Self> {
    let database = app.database();
    let quizzes = database.count_quizzes_with_source(source)?;
    let mut correct_quiz_answers = 0;
    let mut quiz_accuracy = 0.0;

    if quizzes > 0 {
      correct_quiz_answers = database.count_correct_quizzes_with_source(source)?;
      quiz_accuracy = (correct_quiz_answers as f64) / (quizzes as f64);
    }

    Ok(Self {
      source,
      quizzes,
      correct_quiz_answers,
      quiz_accuracy,
    })
  }

  pub fn quizzes(&self) -> u64 {
    self.quizzes
  }
}
