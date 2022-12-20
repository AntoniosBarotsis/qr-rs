use image::ImageError;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
  SizeError(String),
  QrError(String),
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

impl ToString for Error {
  fn to_string(&self) -> String {
    match self {
      Self::ImageError(e) | Self::QrError(e) | Self::SizeError(e) => e.clone(),
    }
  }
}
