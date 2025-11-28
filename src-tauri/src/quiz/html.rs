use crate::database::sql_types::KanjiChar;
use crate::http;
use crate::quiz::Quiz;
use anyhow::Result;
use itertools::Itertools;
use scraper::Html;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use url::Url;

pub(super) async fn from_urls(app: AppHandle, urls: Vec<Url>) -> Result<Quiz> {
  let semaphore = Arc::new(Semaphore::new(3));
  let mut set: JoinSet<Result<Vec<KanjiChar>>> = urls
    .into_iter()
    .unique()
    .map(move |url| {
      let semaphore = Arc::clone(&semaphore);
      async move {
        let permit = semaphore.acquire().await?;
        let html = http::get(url).await?;
        drop(permit);

        let document = Html::parse_document(&html);
        let kanjis = document
          .root_element()
          .text()
          .collect::<String>()
          .chars()
          .filter_map(KanjiChar::from_char)
          .collect();

        Ok(kanjis)
      }
    })
    .collect();

  let mut kanjis = Vec::with_capacity(set.len());
  while let Some(kanji) = set.join_next().await {
    kanjis.extend(kanji??);
  }

  Quiz::from_chunk(app, kanjis).await
}
