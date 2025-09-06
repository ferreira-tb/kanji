#[macro_export]
macro_rules! res {
  ($status:ident) => {{
    use axum::body::Body;
    use axum::http::StatusCode;
    use axum::response::Response;

    let status = StatusCode::$status;
    let body = if (status.is_client_error() || status.is_server_error())
      && let Some(reason) = status.canonical_reason()
    {
      Body::new(format!("{status} {reason}"))
    } else {
      Body::empty()
    };

    Response::builder()
      .status(status)
      .body(body)
      .unwrap()
  }};
  ($status:ident, $data:expr) => {{
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    (StatusCode::$status, $data).into_response()
  }};
}
