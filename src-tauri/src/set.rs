use crate::database::sql_types::KanjiChar;
use serde::Serialize;

#[cfg(desktop)]
use {
  crate::kanji::{KanjiStats, search as search_kanji},
  crate::manager::ManagerExt,
  crate::settings::Settings,
  anyhow::Result,
  itertools::Itertools,
  std::path::Path as StdPath,
  tauri::AppHandle,
  tokio::fs::File,
  tokio::io::AsyncWriteExt,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KanjiSet {
  chunks: Box<[KanjiSetChunk]>,
  quizzes: u64,
  correct_quiz_answers: u64,
  quiz_accuracy: f64,
}

#[cfg(desktop)]
impl KanjiSet {
  pub async fn load(app: AppHandle) -> Result<Self> {
    let settings = Settings::get(&app)?;
    let mut kanjis = search_kanji(app.clone()).await?;
    kanjis.sort_by_key(KanjiStats::character);
    kanjis.sort_by_key(KanjiStats::seen);

    let mut chunks = Vec::new();
    let iter = kanjis
      .iter()
      .map(KanjiStats::character)
      .rev()
      .chunks(settings.set_chunk_size);

    let database = app.database();
    for (id, chunk) in (1u32..).zip(&iter) {
      let id = KanjiSetChunkId(id);
      let kanjis = chunk
        .into_iter()
        .collect_vec()
        .into_boxed_slice();

      let quizzes = database.count_quizzes_in(&kanjis)?;
      let mut correct_quiz_answers = 0;
      let mut quiz_accuracy = 0.0;

      if quizzes > 0 {
        correct_quiz_answers = database.count_correct_quiz_answers_in(&kanjis)?;
        quiz_accuracy = (correct_quiz_answers as f64) / (quizzes as f64);
      }

      chunks.push(KanjiSetChunk {
        id,
        kanjis,
        quizzes,
        correct_quiz_answers,
        quiz_accuracy,
      });
    }

    let quizzes = chunks
      .iter()
      .fold(0u64, |acc, chunk| acc.saturating_add(chunk.quizzes));

    let correct_quiz_answers = chunks.iter().fold(0u64, |acc, chunk| {
      acc.saturating_add(chunk.correct_quiz_answers)
    });

    let quiz_accuracy =
      if quizzes > 0 { (correct_quiz_answers as f64) / (quizzes as f64) } else { 0.0 };

    Ok(Self {
      chunks: chunks.into_boxed_slice(),
      quizzes,
      correct_quiz_answers,
      quiz_accuracy,
    })
  }

  pub async fn export(self, app: AppHandle, folder: &StdPath) -> Result<()> {
    let settings = Settings::get(&app)?;
    let sets = KanjiSet::load(app).await?;
    let path = folder.join(settings.set_file_name.as_ref());
    let mut file = File::create(path).await?;

    for chunk in &sets.chunks {
      let chunk = chunk.kanjis.iter().join("");
      let bytes = chunk.bytes().collect_vec();
      file.write_all(&bytes).await?;
      file.write_all(b"\n").await?;
    }

    file.flush().await?;

    Ok(())
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KanjiSetChunk {
  id: KanjiSetChunkId,
  kanjis: Box<[KanjiChar]>,
  quizzes: u64,
  correct_quiz_answers: u64,
  quiz_accuracy: f64,
}

#[derive(Debug, Serialize)]
pub struct KanjiSetChunkId(u32);
