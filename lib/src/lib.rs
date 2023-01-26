//! This crate provides an easy way of generating QR Codes with a logo overlay in the center.
//! It was developed for [GDSC Delft](https://gdsc.community.dev/delft-university-of-technology/).
//!
//! It uses [`fast-qr`](https://github.com/erwanvivien/fast_qr) which makes is
//! [pretty fast](https://github.com/erwanvivien/fast_qr#benchmarks).
//! This crate also provides its own benchmarks.
//!
//! This crate exposes the [`QrCodeBuilder`] struct which generates the QR Codes.
//!
//! A usage example in the form of an [Actix](actix.rs) web-server can be found
//! [here](https://github.com/AntoniosBarotsis/qr-rs/tree/master/server).

pub mod error;

use std::io::Cursor;

use error::Error;
use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use image::{imageops, ImageFormat};
use image::{io::Reader as ImageReader, GenericImageView};
use image::{DynamicImage, Rgba};

/// The default QR Code size.
pub const DEFAULT_SIZE: u32 = 600;
/// Minimum QR Code size.
pub const SIZE_MIN: u32 = 200;
/// Maximum QR Code size.
pub const SIZE_MAX: u32 = 1000;

const BLACK: [u8; 4] = [0, 0, 0, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];
const TRANSPARENT: [u8; 4] = [255, 255, 255, 0];

/// Wrapper around [Rgba] but without the `a` value.
#[derive(Debug, Clone, Copy)]
pub struct Rgb(pub [u8; 3]);

impl From<Rgb> for Rgba<u8> {
  fn from(val: Rgb) -> Self {
    let tmp = val.0;
    let rgba = [tmp[0], tmp[1], tmp[2], 255];
    Self(rgba)
  }
}

/// Builder for generating QR Codes.
///
///
/// Generates a QR Code in the form of a [`Result<Vec<u8>, Error>`] and overlay a logo on top of it.
/// The [`Vec<u8>`] is generated according to [`image::ImageOutputFormat::Png`].
///
/// ## Argument requirements
///
/// - The `content` should not be empty.
/// - The `size` should be between [`SIZE_MIN`] and [`SIZE_MAX`] (their values might change in
/// future releases).
///
/// Note that these are only checked in [`QrCodeBuilder::build`].
///
/// ## Defaults
///
/// - `bg_color color` defaults to white.
/// - `size` defaults to [`DEFAULT_SIZE`].
///
/// ## Examples
///
/// ```
/// let logo = include_bytes!("../../assets/logo.png");
/// use qr_rs_lib::{QrCodeBuilder, Rgb};
///
/// let qr_code = QrCodeBuilder::new("github.com", logo)
///   .with_size(600)
///   .with_bg_color(Rgb([255, 0, 0]))
///   .build();
///
/// assert!(matches!(qr_code, Ok(_)));
/// ```
#[derive(Debug)]
pub struct QrCodeBuilder<'a, 'b> {
  content: &'a str,
  size: Option<u32>,
  bg_color: Option<Rgb>,
  logo: &'b [u8],
}

impl<'a, 'b> QrCodeBuilder<'a, 'b> {
  pub const fn new(content: &'a str, logo: &'b [u8]) -> QrCodeBuilder<'a, 'b> {
    Self {
      content,
      size: None,
      bg_color: None,
      logo,
    }
  }

  /// The QR Codes are always square so `size` is used for both the
  /// height and width.
  pub fn with_size(&mut self, size: u32) -> &mut Self {
    self.size = Some(size);
    self
  }

  /// Sets the background color of the QR Code. The caller is responsible
  /// for ensuring that the end result is readable.
  pub fn with_bg_color(&mut self, bg_color: Rgb) -> &mut Self {
    self.bg_color = Some(bg_color);
    self
  }

  /// Similar to [`QrCodeBuilder::with_bg_color`] but takes an option instead
  /// for convenience.
  pub fn with_some_bg_color(&mut self, bg_color: Option<Rgb>) -> &mut Self {
    self.bg_color = bg_color;
    self
  }

  pub fn build(&self) -> Result<Vec<u8>, Error> {
    let content = self.content;
    let size = self.size.unwrap_or(DEFAULT_SIZE);
    let bg_color = self.bg_color;
    let logo = self.logo;

    generate_qr_code(content, size, bg_color, logo)
  }
}

fn generate_qr_code(
  content: &str,
  size: u32,
  bg_color: Option<Rgb>,
  logo: &[u8],
) -> Result<Vec<u8>, Error> {
  if !(SIZE_MIN..=SIZE_MAX).contains(&size) {
    return Err(Error::InputError(format!(
      "Size should be between {SIZE_MIN} and {SIZE_MAX}."
    )));
  }

  let bg_color = bg_color.map(std::convert::Into::into);

  // Generate QR Code
  let mut qrcode = fast_qr::QRBuilder::new(content.to_owned());

  // Sometimes when the content was too short, the QR code would be invalid because of the logo.
  // To circumvent this, the error correction level is set to high for URLs of length 0 to 35
  // and then reduced to Quartile which suffices.
  let qrcode = match content.len() {
    1..=35 => qrcode.ecl(fast_qr::ECL::H),
    36.. => qrcode.ecl(fast_qr::ECL::Q),
    _ => {
      return Err(Error::InputError(format!(
        "Invalid content length {}",
        content.len()
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

  // Get the logo
  let logo = ImageReader::new(Cursor::new(logo))
    .with_guessed_format()
    .map_err(|_e| Error::InputError("Image should be either PNG or JPEG".to_owned()))?
    .decode()?;

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

  add_logo(&mut img, &logo);

  let mut bytes: Vec<u8> = Vec::new();
  img.write_to(
    &mut std::io::Cursor::new(&mut bytes),
    image::ImageOutputFormat::Png,
  )?;

  Ok(bytes)
}

/// Adds the logo with a white, circular background in the middle of the image.
fn add_logo(img: &mut DynamicImage, logo: &DynamicImage) {
  // Shrink logo to work with the 25% QR Code error correction.
  let logo = logo.resize(
    img.width() / 4,
    img.width() / 4,
    imageops::FilterType::Nearest,
  );

  let img_center = img.width() / 2;
  let logo_center = logo.width() / 2;

  // Turns the logo into a circle.
  let logo = image::ImageBuffer::from_fn(logo.width(), logo.height(), |x, y| {
    if distance(logo_center, x, y) < (f64::from(logo_center)) {
      logo.get_pixel(x, y)
    } else {
      Rgba(TRANSPARENT)
    }
  });

  let logo_bg = image::ImageBuffer::from_fn(img.width(), img.width(), |x, y| {
    // The 3.5 is just a "magic number ✨" that makes the white circle
    // just big enough for my taste.
    if distance(img_center, x, y) < (f64::from(img_center) / 3.7) {
      Rgba(WHITE)
    } else {
      Rgba(TRANSPARENT)
    }
  });

  let x = img_center - (logo.width() / 2);
  let y = img_center - (logo.height() / 2);

  imageops::overlay(img, &logo_bg, 0, 0);
  imageops::overlay(img, &logo, x.into(), y.into());
}

/// Calculates distance between `(c, c)` and `(x, y)`.
fn distance(c: u32, x: u32, y: u32) -> f64 {
  // Casting here is fine as I cast positive values that are nowhere near large enough to overflow.
  #![allow(clippy::cast_possible_wrap)]
  f64::from((c as i32 - x as i32).pow(2) + (c as i32 - y as i32).pow(2)).sqrt()
}
