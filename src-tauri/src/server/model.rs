use crate::database::sql_types::{KanjiChar, Path, SourceId, SourceWeight};
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSourceRequest {
  pub source: Path,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameSourceRequest {
  pub id: SourceId,
  pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSnippetsRequest {
  pub kanji: KanjiChar,
  pub source: Option<SourceId>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSourceWeightRequest {
  pub id: SourceId,
  pub weight: SourceWeight,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleSourceRequest {
  pub id: SourceId,
  pub enabled: bool,
}
