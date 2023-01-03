pub mod error;

use std::io::Cursor;

use error::Error;
use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
pub use image::Rgba;
use image::{imageops, ImageBuffer, ImageFormat};
use image::{io::Reader as ImageReader, DynamicImage};
use once_cell::sync::OnceCell;

const LOGO: &[u8] = include_bytes!("../assets/logo.png");
static LOGO_IMAGE: OnceCell<DynamicImage> = OnceCell::new();
/// The default QR Code size.
pub const DEFAULT_SIZE: u32 = 600;

const BLACK: [u8; 4] = [0, 0, 0, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];

#[derive(Debug)]
pub struct QrCodeBuilder<'a> {
  link: &'a str,
  size: Option<u32>,
  bg_color: Option<Rgba<u8>>,
}

impl<'a> QrCodeBuilder<'a> {
  pub const fn new(link: &'a str) -> QrCodeBuilder<'a> {
    Self {
      link,
      size: None,
      bg_color: None,
    }
  }

  pub fn with_size(&mut self, size: u32) -> &mut Self {
    self.size = Some(size);
    self
  }

  pub fn with_bg_color(&mut self, bg_color: Option<Rgba<u8>>) -> &mut Self {
    self.bg_color = bg_color;
    self
  }

  pub fn build(&self) -> Result<Vec<u8>, Error> {
    let link = self.link;
    let size = self.size.unwrap_or(DEFAULT_SIZE);
    let bg_color = self.bg_color;

    generate_qr_code(link, size, bg_color)
  }
}

/// Generates a QR Code in the form of a `Vec<u8>`.
pub fn generate_qr_code(
  link: &str,
  size: u32,
  bg_color: Option<Rgba<u8>>,
) -> Result<Vec<u8>, Error> {
  // TODO Arbitrary (but sensible) values for now, maybe we need smaller/bigger?
  if !(200..=1000).contains(&size) {
    return Err(Error::InputError(
      "Size should be between 200 and 1000.".to_string(),
    ));
  }

  // Generate QR Code
  let mut qrcode = fast_qr::QRBuilder::new(link.to_owned());

  // Sometimes when the link was too short, the QR code would be invalid because of the logo.
  // To circumvent this, the error correction level is set to high for URLs of length 0 to 35
  // and then reduced to Quartile which suffices.
  let qrcode = match link.len() {
    1..=35 => qrcode.ecl(fast_qr::ECL::H),
    36.. => qrcode.ecl(fast_qr::ECL::Q),
    _ => {
      return Err(Error::InputError(format!(
        "Invalid link length {}",
        link.len()
      )))
    }
  };

  let qrcode = qrcode.build()?;

  // Convert to image
  let img = ImageBuilder::default()
    .shape(Shape::Square)
    .fit_width(size)
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

  // This *should* always run
  if let Some(tmp) = img.as_mut_rgba8() {
    tmp.enumerate_pixels_mut().for_each(|(_x, _y, p)| {
      // Remove greys
      if p.0 > BLACK {
        *p = Rgba(WHITE);
      }

      if let Some(new_bg) = bg_color {
        if p.0 == WHITE {
          *p = new_bg;
        }
      }
    });
  }

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
    let transparent: [u8; 4] = [255, 255, 255, 0];

    // The 3.5 is just a "magic number ✨" that makes the white circle
    // just big enough for me.
    if distance < (f64::from(center) / 3.5) {
      Rgba(WHITE)
    } else {
      Rgba(transparent)
    }
  })
}
