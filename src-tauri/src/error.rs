use serde::Serialize;
use std::fmt::Display;

#[cfg(desktop)]
use axum::response::Response;

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

#[cfg(desktop)]
impl From<Error> for Response {
  fn from(err: Error) -> Self {
    crate::res!(INTERNAL_SERVER_ERROR, err.0)
  }
}
