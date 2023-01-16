#![allow(dead_code, unused_parens)]

mod error;

use std::{fs::File, io::Write};

use clap::Parser;
use common::{hex_to_rgb, logos::Logo};
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
}

fn main() -> Result<(), CliError> {
  let args = Args::parse();

  let logo = Logo::try_from(args.logo)?.into();
  let bg_color = hex_to_rgb(&args.bg_color);

  let qr_code = QrCodeBuilder::new(&args.content, logo)
    .with_size(args.size)
    .with_some_bg_color(bg_color)
    .build()?;

  let mut f = File::create(args.destination)?;
  f.write_all(&qr_code)?;

  Ok(())
}

#[cfg(test)]
mod tests {
  #[test]
  fn verify_cli() {
    use clap::CommandFactory;
    use crate::Args;
    Args::command().debug_assert()
  }
}

