use crate::database::sql_types::KanjiChar;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuizRequest {
  pub kanjis: Vec<KanjiChar>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuizAnswerRequest {
  pub question: KanjiChar,
  pub answer: KanjiChar,
  pub duration: u32,
}
