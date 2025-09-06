use crate::res;
use axum::response::Response;
use serde::Serialize;
use std::fmt::Display;

pub use std::error::Error as StdError;
pub use std::result::Result as StdResult;

pub type CResult<T> = StdResult<T, Error>;
pub type BoxResult<T> = StdResult<T, Box<dyn StdError>>;

#[derive(Debug, Serialize)]
pub struct Error(String);

impl<T: Display> From<T> for Error {
  fn from(value: T) -> Self {
    Self(value.to_string())
  }
}

impl From<Error> for Response {
  fn from(err: Error) -> Self {
    res!(INTERNAL_SERVER_ERROR, err.0)
  }
}
