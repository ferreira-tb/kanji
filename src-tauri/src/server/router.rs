use super::model::*;
use crate::{command, res};
use axum::Router;
use axum::extract::{Json, State};
use axum::response::Response;
use axum::routing::{get, post};
use futures::TryFutureExt;
use tauri::AppHandle;

pub(super) fn create() -> Router<AppHandle> {
  Router::new()
    .route("/quiz", post(create_quiz))
    .route("/quiz/answer", post(create_quiz_answer))
    .route("/set", get(get_set))
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
  command::create_quiz_answer(app, req.question, req.answer, req.duration)
    .map_ok(|()| res!(OK))
    .unwrap_or_else(Response::from)
    .await
}

async fn get_set(State(app): State<AppHandle>) -> Response {
  command::get_set(app)
    .map_ok(|set| res!(OK, Json(set)))
    .unwrap_or_else(Response::from)
    .await
}
