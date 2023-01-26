use std::fmt::Display;

use image::ImageError;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
  /// Represents invalid parameters (such as a really big size or an empty link).
  InputError(String),
  /// Represents an error in the QR Code generation. If this happens there's likely a bug in this
  /// crate.
  QrError(String),
  /// Represents an error in the encoding of the image. If this happens there's likely a bug
  /// in this crate.
  ImageError(String),
}

impl From<fast_qr::qr::QRCodeError> for Error {
  fn from(e: fast_qr::qr::QRCodeError) -> Self {
    match e {
      fast_qr::qr::QRCodeError::EncodedData => {
        Self::QrError("Data if too big to be encoded".to_owned())
      }
      fast_qr::qr::QRCodeError::SpecifiedVersion => {
        Self::QrError("Specified version too small to contain data".to_owned())
      }
    }
  }
}

impl From<png::EncodingError> for Error {
  fn from(e: png::EncodingError) -> Self {
    Self::ImageError(e.to_string())
  }
}

impl From<ImageError> for Error {
  fn from(e: ImageError) -> Self {
    Self::ImageError(e.to_string())
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      Self::ImageError(e) | Self::QrError(e) | Self::InputError(e) => write!(f, "{e}"),
    }
  }
}
