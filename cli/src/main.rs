#![allow(dead_code, unused_parens)]

mod error;
use std::str::FromStr;

use clap::Parser;
use common::{hex_to_rgb, logos::Logo, read_image_bytes_async};
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

  /// The name of the logo to use in the overlay.
  #[arg(short, long, default_value_t = ("google".to_owned()))]
  logo: String,

  /// Path to the logo (must be a valid PNG/JPEG).
  #[arg(long, visible_alias("path"))]
  logo_source: Option<String>,

  /// URL to the logo (must be a valid PNG/JPEG).
  #[arg(long, visible_alias("web"))]
  logo_web_source: Option<String>,

  /// The background color of the QR Code (in hex).
  #[arg(short = 'c', long, default_value_t = ("FFFFFF".to_owned()))]
  bg_color: String,

  /// The background color of the logo (in hex).
  #[arg(long, visible_alias("lc"), default_value_t = ("FFFFFF".to_owned()))]
  logo_bg_color: String,
}

#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> Result<(), CliError> {
  // The command line args are passed here as an iterable
  // instead of parsing them directly to facilitate testing.
  qrg(std::env::args().collect::<Vec<_>>()).await
}

async fn qrg(args: Vec<String>) -> Result<(), CliError> {
  let args = Args::parse_from(args);

  // logo_source > logo_web_source > logo
  let logo: Vec<u8> = match (&args.logo_source, &args.logo_web_source) {
    // If logo_source exists (ignoring logo_web_source)
    (Some(l), Some(_) | None) => read_file(l)?,
    // If logo_web_source exists
    (None, Some(l)) => read_image_bytes_async(l)
      .await
      .ok_or_else(|| CliError::IoError(format!("Error fetching image from '{l}'")))?,
    // If neither, use logo
    (None, None) => Logo::from_str(&args.logo)?.into(),
  };

  // TODO Maybe these should throw an error if they are not valid hex strings for better UX.
  let bg_color = hex_to_rgb(&args.bg_color);
  let logo_bg_color = hex_to_rgb(&args.logo_bg_color);

  #[allow(unused_variables)] // Silence warning for cfg(test)
  let qr_code = QrCodeBuilder::new(&args.content, &logo)
    .with_size(args.size)
    .with_some_bg_color(bg_color)
    .with_some_logo_bg_color(logo_bg_color)
    .build()?;

  // Don't save any images when running tests
  #[cfg(not(test))]
  {
    use std::{fs::File, io::Write};

    let mut f = File::create(args.destination)?;
    f.write_all(&qr_code)?;
  }

  Ok(())
}

/// Reads the file on the given path and returns its bytes.
fn read_file(logo_source: &str) -> Result<Vec<u8>, CliError> {
  let bytes = std::fs::read(logo_source)?;

  Ok(bytes)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
  use crate::qrg;

  #[test]
  fn verify_cli() {
    use crate::Args;
    use clap::CommandFactory;
    Args::command().debug_assert();
  }

  /// This is **COMPLETELY** unnecessary and overkill but I wanted to try and make a macro.
  ///
  /// This takes in string slices and generates a vector of [`String`]s to be used in
  /// [`Args::parse_from`] for testing.
  ///
  /// It's supposed to emulate command line arguments, as the name suggests.
  macro_rules! cli_args {
    ($($x: expr),*) => {{
      // First arg is ignored, command line args start at index 1.
      let vector: Vec<String> = vec!["".to_owned(), $($x.to_owned()),*];
      vector
    }}
  }

  #[tokio::test]
  async fn generate_simple() {
    let args = cli_args!("content");
    let res = qrg(args).await;
    assert!(res.is_ok());
  }

  #[tokio::test]
  async fn generate_destination() {
    let args = cli_args!("content", "--destination", "res.png");
    let res = qrg(args).await;
    assert!(res.is_ok());
  }

  #[tokio::test]
  async fn generate_size() {
    let args = cli_args!("content", "--size", "1000");
    let res = qrg(args).await;
    assert!(res.is_ok());
  }

  #[tokio::test]
  async fn generate_logo() {
    let args = cli_args!("content", "--logo", "google");
    let res = qrg(args).await;
    assert!(res.is_ok());
  }

  #[tokio::test]
  async fn generate_path() {
    let args = cli_args!("content", "--path", "../assets/example.png");
    let res = qrg(args).await;
    assert!(res.is_ok());
  }

  #[tokio::test]
  async fn generate_path_invalid() {
    let args = cli_args!("content", "--path", "../assets/idk.png");
    let res = qrg(args).await;
    assert!(res.is_err());
  }

  #[tokio::test]
  async fn generate_web() {
    let args = cli_args!(
      "content",
      "--web",
      "https://github.com/AntoniosBarotsis/qr-rs/raw/master/assets/example.png"
    );
    let res = qrg(args).await;
    assert!(res.is_ok());
  }

  #[tokio::test]
  async fn generate_web_invalid() {
    let args = cli_args!("content", "--web", "https://github.com/");
    let res = qrg(args).await;
    assert!(res.is_err());
  }

  #[tokio::test]
  async fn generate_bg_color() {
    let args = cli_args!("content", "--bg-color", "FF00FF");
    let res = qrg(args).await;
    assert!(res.is_ok());
  }

  #[tokio::test]
  async fn generate_logo_bg_color() {
    let args = cli_args!("content", "--logo-bg-color", "FF00FF");
    let res = qrg(args).await;
    assert!(res.is_ok());
  }

  #[tokio::test]
  async fn all_params() {
    let args = cli_args!(
      "content",
      "--destination",
      "res.png",
      "--size",
      "1000",
      "--logo",
      "google",
      "--logo-source",
      "../assets/example.png",
      "--logo-web-source",
      "https://github.com/AntoniosBarotsis/qr-rs/raw/master/assets/example.png",
      "--bg-color",
      "00FF00",
      "--logo-bg-color",
      "FF00FF"
    );
    let res = qrg(args).await;
    assert!(res.is_ok());
  }
}
