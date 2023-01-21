#![allow(dead_code, unused_parens)]

mod error;
use std::{
  fs::File,
  io::{prelude::Read, Write},
};

use clap::Parser;
use common::{hex_to_rgb, logos::Logo, read_image_bytes};
use error::CliError;
use qr_rs_lib::{QrCodeBuilder, DEFAULT_SIZE};

/// A CLI for generating QR Codes with a logo overlay.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  /// The text the qr code should contain.
  #[arg()]
  content: String,

  /// The filename the QR Code should be saved in.
  #[arg(short, long, default_value_t = ("out.png".to_owned()))]
  destination: String,

  /// The size of the QR Code
  #[arg(short, long, default_value_t = DEFAULT_SIZE)]
  size: u32,

  /// The background color of the QR Code (in hex).
  #[arg(short = 'c', long, default_value_t = ("FFFFFF".to_owned()))]
  bg_color: String,

  /// The name of the logo to use in the overlay.
  #[arg(short, long, default_value_t = ("google".to_owned()))]
  logo: String,

  // TODO Alias?
  /// Path to the logo (must be a valid PNG/JPEG).
  #[arg(long)]
  logo_source: Option<String>,

  // TODO Alias?
  /// URL to the logo (must be a valid PNG/JPEG).
  #[arg(long)]
  logo_web_source: Option<String>,
}

fn main() -> Result<(), CliError> {
  let args = Args::parse();

  // TODO Looks ugly
  let logo: Vec<u8> = match (&args.logo_source, &args.logo_web_source) {
    (Some(l), Some(_)) | (Some(l), None) => read_file(l)?,
    (None, Some(l)) => {
      read_image_bytes(l).ok_or_else(|| CliError::IoError(format!("Error fetching image from '{l}'")))?
    }
    (None, None) => Logo::try_from(&args.logo)?.into(),
  };

  let bg_color = hex_to_rgb(&args.bg_color);

  let qr_code = QrCodeBuilder::new(&args.content, &logo)
    .with_size(args.size)
    .with_some_bg_color(bg_color)
    .build()?;

  let mut f = File::create(args.destination)?;
  f.write_all(&qr_code)?;

  Ok(())
}

/// Reads the file on the given path and returns its bytes.
fn read_file(logo_source: &str) -> Result<Vec<u8>, CliError> {
  let mut f = File::open(logo_source)?;
  let mut buffer = Vec::new();
  let _ = f.read_to_end(&mut buffer)?;

  Ok(buffer)
}

#[cfg(test)]
mod tests {
  #[test]
  fn verify_cli() {
    use crate::Args;
    use clap::CommandFactory;
    Args::command().debug_assert()
  }
}
