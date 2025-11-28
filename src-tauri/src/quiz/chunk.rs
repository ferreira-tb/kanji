use crate::database::model::source::Source;
use crate::database::sql_types::KanjiChar;
use crate::kanji::is_kanji;
use crate::manager::ManagerExt;
use crate::quiz::{MARUMARU, Quiz, QuizQuestion};
use crate::snippet::blocking_search_with_options as search_snippet;
use anyhow::Result;
use futures::future::BoxFuture;
use itertools::Itertools;
use rand::seq::{IteratorRandom, SliceRandom};
use std::sync::Arc;
use tauri::AppHandle;
use tauri::async_runtime::spawn_blocking;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

pub(super) async fn with_sources(
  app: AppHandle,
  kanjis: Vec<KanjiChar>,
  sources: Vec<Source>,
) -> Result<Quiz> {
  let db = app.database();
  let chars = Arc::from(db.get_kanji_chars()?);
  let sources = Arc::from(sources);

  let mut set: JoinSet<Result<Option<QuizQuestion>>> = kanjis
    .into_iter()
    .filter(|kanji| is_kanji(**kanji))
    .unique()
    .map(make_questions(app, chars, sources))
    .collect();

  let mut questions = Vec::with_capacity(set.len());
  while let Some(question) = set.join_next().await {
    if let Some(question) = question?? {
      questions.push(question);
    }
  }

  questions.shuffle(&mut rand::rng());

  Ok(Quiz(questions))
}

fn make_questions(
  app: AppHandle,
  chars: Arc<[KanjiChar]>,
  sources: Arc<[Source]>,
) -> impl Fn(KanjiChar) -> BoxFuture<'static, Result<Option<QuizQuestion>>> {
  let semaphore = Arc::new(Semaphore::new(100));
  move |kanji| {
    let app = app.clone();
    let chars = Arc::clone(&chars);
    let sources = Arc::clone(&sources);
    let semaphore = Arc::clone(&semaphore);

    Box::pin(async move {
      let permit = semaphore.acquire().await?;
      let snippet = spawn_blocking(move || {
        search_snippet(&app, kanji)
          .sources(&sources)
          .limit(1)
          .shuffle(true)
          .call()
      });

      let mut question = None;
      if let Some(snippet) = snippet.await??.pop() {
        let censored = snippet.content().replace(*kanji, MARUMARU);
        question = Some(QuizQuestion {
          snippet,
          censored,
          answer: kanji,
          options: pick_options(kanji, &chars),
        });
      }

      drop(permit);

      Ok(question)
    })
  }
}

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
