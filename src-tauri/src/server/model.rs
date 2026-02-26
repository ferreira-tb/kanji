use crate::database::sql_types::{
  BookmarkId,
  KanjiChar,
  KanjiSetChunkId,
  SourceGroupId,
  SourceId,
  SourceWeight,
  SqlPath,
};
use crate::quiz::QuizKind;
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
  pub kind: QuizKind,
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
pub struct CreateQuizChunkHistoryEntry {
  pub id: KanjiSetChunkId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSourceRequest {
  pub source: SqlPath,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSourceGroupRequest {
  pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceRequest {
  pub id: SourceId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceGroupRequest {
  pub id: SourceGroupId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceGroupSourceIdsRequest {
  pub id: SourceGroupId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceGroupSourcesRequest {
  pub id: SourceGroupId,
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
pub struct RemoveSourceGroupRequest {
  pub id: SourceGroupId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameSourceRequest {
  pub id: SourceId,
  pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameSourceGroupRequest {
  pub id: SourceGroupId,
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
pub struct SetSourceGroupSourcesRequest {
  pub id: SourceGroupId,
  pub sources: Vec<SourceId>,
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
