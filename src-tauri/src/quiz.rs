use crate::database::sql_types::KanjiChar;
use crate::manager::ManagerExt;
use crate::settings::Settings;
use crate::snippet::{self, Snippet};
use anyhow::Result;
use itertools::Itertools;
use rand::seq::{IndexedRandom, IteratorRandom, SliceRandom};
use serde::Serialize;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::async_runtime::spawn_blocking;
use tokio::sync::{Mutex, Semaphore};
use tokio::task::JoinSet;

const MARUMARU: &str = "◯";

#[derive(Serialize)]
pub struct Quiz(Vec<QuizQuestion>);

impl Quiz {
  pub async fn new<I>(app: AppHandle, kanjis: I) -> Result<Self>
  where
    I: IntoIterator<Item = KanjiChar>,
  {
    let db = app.database();
    let chars = Arc::from(db.get_kanji_chars()?);
    let sources = Arc::from(db.get_enabled_sources()?);
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

  pub async fn random(app: AppHandle) -> Result<Self> {
    let settings = Settings::get(&app)?;
    let kanjis = app
      .database()
      .get_kanji_chars()?
      .choose_multiple(&mut rand::rng(), settings.set_chunk_size)
      .copied()
      .collect_vec();

    Self::new(app, kanjis).await
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizQuestion {
  snippet: Snippet,
  censored: String,
  answer: KanjiChar,
  options: Vec<KanjiChar>,
}
