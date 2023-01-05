use core::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug)]
pub enum Error {
  InvalidColor,
  Generic(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      Self::InvalidColor => write!(f, "Invalid color"),
      Self::Generic(e) => write!(f, "{e}"),
    }
  }
}

impl ResponseError for Error {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(StatusCode::BAD_REQUEST).json(self.to_string())
  }
}
