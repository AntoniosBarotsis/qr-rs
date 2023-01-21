use common::logos::InvalidLogo;
use qr_rs_lib::error::Error;

#[derive(Debug)]
pub enum CliError {
  InvalidColor(String),
  InvalidLogo(String),
  IoError(String),
  Internal(String),
}

impl From<InvalidLogo> for CliError {
  fn from(value: InvalidLogo) -> Self {
    Self::InvalidLogo(value.0)
  }
}

impl From<Error> for CliError {
  fn from(value: Error) -> Self {
    Self::Internal(value.to_string())
  }
}

impl From<std::io::Error> for CliError {
  fn from(value: std::io::Error) -> Self {
    Self::IoError(value.to_string())
  }
}
