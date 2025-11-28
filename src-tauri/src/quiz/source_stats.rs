use crate::database::sql_types::SourceId;
use serde::Serialize;

#[cfg(desktop)]
use {crate::manager::ManagerExt, anyhow::Result, tauri::AppHandle};

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
    let db = app.database();
    let quizzes = db.count_quizzes_with_source(source)?;
    let mut correct_quiz_answers = 0;
    let mut quiz_accuracy = 0.0;

    if quizzes > 0 {
      correct_quiz_answers = db.count_correct_quizzes_with_source(source)?;
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
