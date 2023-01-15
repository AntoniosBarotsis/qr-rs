use core::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug)]
pub enum Error {
  InvalidLogo(String),
  InvalidColor,
  Lib(Box<qr_rs_lib::error::Error>),
}

impl From<qr_rs_lib::error::Error> for Error {
  fn from(e: qr_rs_lib::error::Error) -> Self {
    Self::Lib(Box::new(e))
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      Self::InvalidColor => write!(f, "Invalid color"),
      Self::Lib(e) => write!(f, "{}", *e),
      Self::InvalidLogo(s) => write!(f, "Invalid logo: {s}"),
    }
  }
}

impl ResponseError for Error {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(StatusCode::BAD_REQUEST).json(self.to_string())
  }
}
