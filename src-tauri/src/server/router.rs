use super::model::*;
use crate::{command, res};
use axum::Router;
use axum::extract::{Json, State};
use axum::response::Response;
use axum::routing::{get, post};
use futures::TryFutureExt;
use tauri::AppHandle;
use tower_http::cors::{Any, CorsLayer};

#[rustfmt::skip]
pub(super) fn create() -> Router<AppHandle> {
  let cors = CorsLayer::new()
    .allow_methods(Any)
    .allow_headers(Any)
    .allow_origin(Any);

  Router::new()
    .route("/clear-quiz-chunk-history", get(clear_quiz_chunk_history))
    .route("/create-bookmark", post(create_bookmark))
    .route("/create-quiz", post(create_quiz))
    .route("/create-quiz-answer", post(create_quiz_answer))
    .route("/create-quiz-chunk-history-entry", post(create_quiz_chunk_history_entry))
    .route("/create-source", post(create_source))
    .route("/create-source-group", post(create_source_group))
    .route("/get-bookmarks", get(get_bookmarks))
    .route("/get-quiz-answers", get(get_quiz_answers))
    .route("/get-quiz-chunk-history-entries", get(get_quiz_chunk_history_entries))
    .route("/get-quiz-source-stats", get(get_quiz_source_stats))
    .route("/get-set", get(get_set))
    .route("/get-source", post(get_source))
    .route("/get-source-group", post(get_source_group))
    .route("/get-source-group-ids", get(get_source_group_ids))
    .route("/get-source-group-source-ids", post(get_source_group_source_ids))
    .route("/get-source-group-sources", post(get_source_group_sources))
    .route("/get-source-groups", get(get_source_groups))
    .route("/get-sources", get(get_sources))
    .route("/remove-bookmark", post(remove_bookmark))
    .route("/remove-source", post(remove_source))
    .route("/remove-source-group", post(remove_source_group))
    .route("/rename-source", post(rename_source))
    .route("/rename-source-group", post(rename_source_group))
    .route("/search-kanji", get(search_kanji))
    .route("/search-snippets", post(search_snippets))
    .route("/set-source-group-sources", post(set_source_group_sources))
    .route("/set-source-weight", post(set_source_weight))
    .route("/toggle-source", post(toggle_source))
    .layer(cors)
}

async fn clear_quiz_chunk_history(State(app): State<AppHandle>) -> Response {
  command::quiz::clear_quiz_chunk_history(app)
    .map_ok(|rows| res!(OK, Json(rows)))
    .unwrap_or_else(Response::from)
    .await
}

async fn create_bookmark(
  State(app): State<AppHandle>,
  Json(req): Json<CreateBookmarkRequest>,
) -> Response {
  command::bookmark::create_bookmark(app, req.snippet)
    .map_ok(|id| res!(CREATED, Json(id)))
    .unwrap_or_else(Response::from)
    .await
}

async fn create_quiz(State(app): State<AppHandle>, Json(req): Json<CreateQuizRequest>) -> Response {
  command::quiz::create_quiz(app, req.kind)
    .map_ok(|quiz| res!(CREATED, Json(quiz)))
    .unwrap_or_else(Response::from)
    .await
}

async fn create_quiz_answer(
  State(app): State<AppHandle>,
  Json(req): Json<CreateQuizAnswerRequest>,
) -> Response {
  command::quiz::create_quiz_answer(app, req.question, req.answer, req.source)
    .map_ok(|id| res!(CREATED, Json(id)))
    .unwrap_or_else(Response::from)
    .await
}

async fn create_quiz_chunk_history_entry(
  State(app): State<AppHandle>,
  Json(req): Json<CreateQuizChunkHistoryEntry>,
) -> Response {
  command::quiz::create_quiz_chunk_history_entry(app, req.id)
    .map_ok(|()| res!(CREATED))
    .unwrap_or_else(Response::from)
    .await
}

async fn create_source(
  State(app): State<AppHandle>,
  Json(req): Json<CreateSourceRequest>,
) -> Response {
  command::source::create_source(app, req.source)
    .map_ok(|id| res!(CREATED, Json(id)))
    .unwrap_or_else(Response::from)
    .await
}

async fn create_source_group(
  State(app): State<AppHandle>,
  Json(req): Json<CreateSourceGroupRequest>,
) -> Response {
  command::source_group::create_source_group(app, req.name)
    .map_ok(|id| res!(CREATED, Json(id)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_bookmarks(State(app): State<AppHandle>) -> Response {
  command::bookmark::get_bookmarks(app)
    .map_ok(|bookmarks| res!(OK, Json(bookmarks)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_quiz_answers(State(app): State<AppHandle>) -> Response {
  command::quiz::get_quiz_answers(app)
    .map_ok(|answers| res!(OK, Json(answers)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_quiz_chunk_history_entries(State(app): State<AppHandle>) -> Response {
  command::quiz::get_quiz_chunk_history_entries(app)
    .map_ok(|entries| res!(OK, Json(entries)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_quiz_source_stats(State(app): State<AppHandle>) -> Response {
  command::quiz::get_quiz_source_stats(app)
    .map_ok(|stats| res!(OK, Json(stats)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_set(State(app): State<AppHandle>) -> Response {
  command::kanji::get_set(app)
    .map_ok(|set| res!(OK, Json(set)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_source(State(app): State<AppHandle>, Json(req): Json<GetSourceRequest>) -> Response {
  command::source::get_source(app, req.id)
    .map_ok(|source| res!(OK, Json(source)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_source_group(
  State(app): State<AppHandle>,
  Json(req): Json<GetSourceGroupRequest>,
) -> Response {
  command::source_group::get_source_group(app, req.id)
    .map_ok(|group| res!(OK, Json(group)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_source_group_ids(State(app): State<AppHandle>) -> Response {
  command::source_group::get_source_group_ids(app)
    .map_ok(|ids| res!(OK, Json(ids)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_source_group_source_ids(
  State(app): State<AppHandle>,
  Json(req): Json<GetSourceGroupSourceIdsRequest>,
) -> Response {
  command::source_group::get_source_group_source_ids(app, req.id)
    .map_ok(|ids| res!(OK, Json(ids)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_source_group_sources(
  State(app): State<AppHandle>,
  Json(req): Json<GetSourceGroupSourcesRequest>,
) -> Response {
  command::source_group::get_source_group_sources(app, req.id)
    .map_ok(|sources| res!(OK, Json(sources)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_source_groups(State(app): State<AppHandle>) -> Response {
  command::source_group::get_source_groups(app)
    .map_ok(|groups| res!(OK, Json(groups)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_sources(State(app): State<AppHandle>) -> Response {
  command::source::get_sources(app)
    .map_ok(|sources| res!(OK, Json(sources)))
    .unwrap_or_else(Response::from)
    .await
}

async fn remove_bookmark(
  State(app): State<AppHandle>,
  Json(req): Json<RemoveBookmarkRequest>,
) -> Response {
  command::bookmark::remove_bookmark(app, req.id)
    .map_ok(|rows| res!(OK, Json(rows)))
    .unwrap_or_else(Response::from)
    .await
}

async fn remove_source(
  State(app): State<AppHandle>,
  Json(req): Json<RemoveSourceRequest>,
) -> Response {
  command::source::remove_source(app, req.id)
    .map_ok(|rows| res!(OK, Json(rows)))
    .unwrap_or_else(Response::from)
    .await
}

async fn remove_source_group(
  State(app): State<AppHandle>,
  Json(req): Json<RemoveSourceGroupRequest>,
) -> Response {
  command::source_group::remove_source_group(app, req.id)
    .map_ok(|rows| res!(OK, Json(rows)))
    .unwrap_or_else(Response::from)
    .await
}

async fn rename_source(
  State(app): State<AppHandle>,
  Json(req): Json<RenameSourceRequest>,
) -> Response {
  command::source::rename_source(app, req.id, req.name)
    .map_ok(|()| res!(OK))
    .unwrap_or_else(Response::from)
    .await
}

async fn rename_source_group(
  State(app): State<AppHandle>,
  Json(req): Json<RenameSourceGroupRequest>,
) -> Response {
  command::source_group::rename_source_group(app, req.id, req.name)
    .map_ok(|()| res!(OK))
    .unwrap_or_else(Response::from)
    .await
}

async fn search_kanji(State(app): State<AppHandle>) -> Response {
  command::kanji::search_kanji(app)
    .map_ok(|stats| res!(OK, Json(stats)))
    .unwrap_or_else(Response::from)
    .await
}

async fn search_snippets(
  State(app): State<AppHandle>,
  Json(req): Json<SearchSnippetsRequest>,
) -> Response {
  command::kanji::search_snippets(app, req.kanji, req.source)
    .map_ok(|snippets| res!(OK, Json(snippets)))
    .unwrap_or_else(Response::from)
    .await
}

async fn set_source_group_sources(
  State(app): State<AppHandle>,
  Json(req): Json<SetSourceGroupSourcesRequest>,
) -> Response {
  command::source_group::set_source_group_sources(app, req.id, req.sources)
    .map_ok(|()| res!(OK))
    .unwrap_or_else(Response::from)
    .await
}

async fn set_source_weight(
  State(app): State<AppHandle>,
  Json(req): Json<SetSourceWeightRequest>,
) -> Response {
  command::source::set_source_weight(app, req.id, req.weight)
    .map_ok(|()| res!(OK))
    .unwrap_or_else(Response::from)
    .await
}

async fn toggle_source(
  State(app): State<AppHandle>,
  Json(req): Json<ToggleSourceRequest>,
) -> Response {
  command::source::toggle_source(app, req.id, req.enabled)
    .map_ok(|()| res!(OK))
    .unwrap_or_else(Response::from)
    .await
}
