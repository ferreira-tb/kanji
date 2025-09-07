use super::model::*;
use crate::{command, res};
use axum::Router;
use axum::extract::{Json, State};
use axum::response::Response;
use axum::routing::{get, post};
use futures::TryFutureExt;
use tauri::AppHandle;
use tower_http::cors::{Any, CorsLayer};

pub(super) fn create() -> Router<AppHandle> {
  let cors = CorsLayer::new()
    .allow_methods(Any)
    .allow_headers(Any)
    .allow_origin(Any);

  Router::new()
    .route("/create-quiz", post(create_quiz))
    .route("/create-quiz-answer", post(create_quiz_answer))
    .route("/create-source", post(create_source))
    .route("/get-set", get(get_set))
    .route("/get-sources", get(get_sources))
    .route("/rename-source", post(rename_source))
    .route("/search-kanji", get(search_kanji))
    .route("/search-snippets", post(search_snippets))
    .route("/set-source-weight", post(set_source_weight))
    .route("/toggle-source", post(toggle_source))
    .layer(cors)
}

async fn create_quiz(State(app): State<AppHandle>, Json(req): Json<CreateQuizRequest>) -> Response {
  command::create_quiz(app, req.kanjis)
    .map_ok(|quiz| res!(OK, Json(quiz)))
    .unwrap_or_else(Response::from)
    .await
}

async fn create_quiz_answer(
  State(app): State<AppHandle>,
  Json(req): Json<CreateQuizAnswerRequest>,
) -> Response {
  command::create_quiz_answer(app, req.question, req.answer)
    .map_ok(|()| res!(OK))
    .unwrap_or_else(Response::from)
    .await
}

async fn create_source(
  State(app): State<AppHandle>,
  Json(req): Json<CreateSourceRequest>,
) -> Response {
  command::create_source(app, req.source)
    .map_ok(|id| res!(OK, Json(id)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_set(State(app): State<AppHandle>) -> Response {
  command::get_set(app)
    .map_ok(|set| res!(OK, Json(set)))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_sources(State(app): State<AppHandle>) -> Response {
  command::get_sources(app)
    .map_ok(|sources| res!(OK, Json(sources)))
    .unwrap_or_else(Response::from)
    .await
}

async fn rename_source(
  State(app): State<AppHandle>,
  Json(req): Json<RenameSourceRequest>,
) -> Response {
  command::rename_source(app, req.id, req.name)
    .map_ok(|()| res!(OK))
    .unwrap_or_else(Response::from)
    .await
}

async fn search_kanji(State(app): State<AppHandle>) -> Response {
  command::search_kanji(app)
    .map_ok(|stats| res!(OK, Json(stats)))
    .unwrap_or_else(Response::from)
    .await
}

async fn search_snippets(
  State(app): State<AppHandle>,
  Json(req): Json<SearchSnippetsRequest>,
) -> Response {
  command::search_snippets(app, req.kanji, req.source)
    .map_ok(|snippets| res!(OK, Json(snippets)))
    .unwrap_or_else(Response::from)
    .await
}

async fn set_source_weight(
  State(app): State<AppHandle>,
  Json(req): Json<SetSourceWeightRequest>,
) -> Response {
  command::set_source_weight(app, req.id, req.weight)
    .map_ok(|()| res!(OK))
    .unwrap_or_else(Response::from)
    .await
}

async fn toggle_source(
  State(app): State<AppHandle>,
  Json(req): Json<ToggleSourceRequest>,
) -> Response {
  command::toggle_source(app, req.id, req.enabled)
    .map_ok(|()| res!(OK))
    .unwrap_or_else(Response::from)
    .await
}
