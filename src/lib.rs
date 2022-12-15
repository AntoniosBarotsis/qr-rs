use std::io::Cursor;

use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use image::{imageops, ImageBuffer, ImageError, ImageFormat, Rgba};
use image::{io::Reader as ImageReader, DynamicImage};
use once_cell::sync::OnceCell;

const LOGO: &[u8] = include_bytes!("../assets/logo.png");
static LOGO_IMAGE: OnceCell<DynamicImage> = OnceCell::new();

pub fn generate_qr_code(link: &str) -> Result<Vec<u8>, Error> {
  // Generate QR Code
  let mut qrcode = fast_qr::QRBuilder::new(link.to_owned());

  // Sometimes when the link was too short, the QR code would be invalid because of the logo.
  // To circumvent this, the error correction level is set to high for URLs of length 0 to 35
  // and then reduced to Quartile which suffices.
  let qrcode = match link.len() {
    1..=35 => qrcode.ecl(fast_qr::ECL::H),
    36.. => qrcode.ecl(fast_qr::ECL::Q),
    _ => {
      return Err(Error::SizeError(format!(
        "Invalid link length {}",
        link.len()
      )))
    }
  };

  let qrcode = qrcode.build()?;

  // Convert to image
  let img = ImageBuilder::default()
    .shape(Shape::Square)
    .fit_width(600)
    .to_pixmap(&qrcode)
    .encode_png()?;

  // Get or init the logo
  let logo = LOGO_IMAGE.get_or_init(|| {
    let mut reader = ImageReader::new(Cursor::new(LOGO));
    reader.set_format(ImageFormat::Png);

    reader.decode().expect("File should be decodable")
  });

  // Convert QR Code to a PNG
  let mut img = ImageReader::new(std::io::Cursor::new(&img));
  img.set_format(ImageFormat::Png);
  let mut img = img.decode()?;

  // Overlay logo on top of QR code
  let center = img.width() / 2;
  let logo = logo.resize(center / 2, center / 2, imageops::FilterType::Nearest);
  let x = center - (logo.width() / 2);
  let y = center - (logo.height() / 2);

  // Create white bg for the logo
  let logo_bg = generate_circular_padding(center);

  // Overlay the logo
  imageops::overlay(&mut img, &logo_bg, 0, 0);
  imageops::overlay(&mut img, &logo, x.into(), y.into());

  let mut bytes: Vec<u8> = Vec::new();
  img.write_to(
    &mut std::io::Cursor::new(&mut bytes),
    image::ImageOutputFormat::Png,
  )?;

  Ok(bytes)
}

fn generate_circular_padding(center: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  // Casting here is fine as I cast positive values that are nowhere near large enough to overflow.
  // This is needed because `center - x` overflows.
  #![allow(clippy::cast_possible_wrap)]

  let casted_center = center as i32;
  image::ImageBuffer::from_fn(center * 2, center * 2, |x, y| {
    let distance =
      f64::from((casted_center - x as i32).pow(2) + (casted_center - y as i32).pow(2)).sqrt();
    let white: [u8; 4] = [255, 255, 255, 255];
    let transparent: [u8; 4] = [255, 255, 255, 0];

    // The 3.5 is just a "magic number ✨" that makes the white circle
    // just big enough for me.
    if distance < (f64::from(center) / 3.5) {
      Rgba(white)
    } else {
      Rgba(transparent)
    }
  })
}

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
