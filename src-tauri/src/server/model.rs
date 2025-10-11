use crate::database::sql_types::{BookmarkId, KanjiChar, Path, SourceId, SourceWeight};
use crate::snippet::Snippet;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookmarkRequest {
  pub snippet: Snippet,
}

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
  pub source: Option<SourceId>,
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
pub struct RemoveBookmarkRequest {
  pub id: BookmarkId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveSourceRequest {
  pub id: SourceId,
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
