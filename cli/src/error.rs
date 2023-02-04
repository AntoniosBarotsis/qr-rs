use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
  #[error("Invalid color \"{0}\". This should be a valid hex value such as \"FFFFFF\".")]
  InvalidColor(String),

  #[error("{0}")]
  InvalidLogo(String),

  #[error("IO Error {0}.")]
  IoError(String),

  #[error("Internal error {0}.")]
  Internal(String),
}
