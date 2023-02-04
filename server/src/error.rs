use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use common::logos::InvalidLogo;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
  #[error("Invalid logo {0}.")]
  InvalidLogo(String),
  #[error("Invalid color .")]
  InvalidColor,
  #[error("{0}")]
  Lib(Box<qr_rs_lib::error::Error>),
}

impl From<qr_rs_lib::error::Error> for ServerError {
  fn from(e: qr_rs_lib::error::Error) -> Self {
    Self::Lib(Box::new(e))
  }
}

impl ResponseError for ServerError {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(StatusCode::BAD_REQUEST).json(self.to_string())
  }
}

impl From<InvalidLogo> for ServerError {
  fn from(value: InvalidLogo) -> Self {
    Self::InvalidLogo(value.0)
  }
}
