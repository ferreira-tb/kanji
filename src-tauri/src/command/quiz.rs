use crate::database::model::quiz_answer::{NewQuizAnswer, QuizAnswer};
use crate::database::model::quiz_chunk_history::{NewQuizChunkHistoryEntry, QuizChunkHistoryEntry};
use crate::database::sql_types::{KanjiChar, KanjiSetChunkId, QuizAnswerId, SourceId};
use crate::error::CResult;
use crate::manager::ManagerExt;
use crate::quiz::{Quiz, QuizKind, QuizSourceStats};
use itertools::Itertools;
use tauri::AppHandle;
use tauri::async_runtime::spawn_blocking;

#[tauri::command]
pub async fn clear_quiz_chunk_history(app: AppHandle) -> CResult<usize> {
  app
    .database()
    .clear_quiz_chunk_history()
    .map_err(Into::into)
}

#[tauri::command]
pub async fn create_quiz(app: AppHandle, kind: QuizKind) -> CResult<Quiz> {
  Quiz::new(app, kind)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn create_quiz_answer(
  app: AppHandle,
  question: KanjiChar,
  answer: KanjiChar,
  source: Option<SourceId>,
) -> CResult<QuizAnswerId> {
  NewQuizAnswer::builder()
    .question(question)
    .answer(answer)
    .maybe_source_id(source)
    .build()
    .create(&app)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn create_quiz_chunk_history_entry(app: AppHandle, id: KanjiSetChunkId) -> CResult<()> {
  NewQuizChunkHistoryEntry::builder(id)
    .build()
    .create(&app)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_quiz_answers(app: AppHandle) -> CResult<Vec<QuizAnswer>> {
  let task = spawn_blocking(move || {
    app
      .database()
      .get_quiz_answers()
      .map_err(Into::into)
  });

  task.await?
}

#[tauri::command]
pub async fn get_quiz_chunk_history_entries(app: AppHandle) -> CResult<Vec<QuizChunkHistoryEntry>> {
  app
    .database()
    .get_quiz_chunk_history_entries()
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_quiz_source_stats(app: AppHandle) -> CResult<Vec<QuizSourceStats>> {
  let task = spawn_blocking(move || {
    let mut stats: Vec<QuizSourceStats> = app
      .database()
      .get_source_ids()?
      .into_iter()
      .map(|source| QuizSourceStats::new(&app, source))
      .try_collect()?;

    stats.retain(|it| it.quizzes() > 0);
    stats.sort_by_key(QuizSourceStats::quizzes);
    stats.reverse();

    Ok(stats)
  });

  task.await?
}
