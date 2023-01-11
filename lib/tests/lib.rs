// use pretty_assertions::assert_eq;
use qr_rs_lib::{error::Error, QrCodeBuilder};

const LOGO: &[u8] = include_bytes!("../../assets/logo.png");

#[test]
fn empty_link() {
  let res = QrCodeBuilder::new("", LOGO)
    .build()
    .expect_err("Empty link should fail.");

  assert!(matches!(res, Error::InputError(_)));
}

#[test]
fn size_too_small() {
  let res = QrCodeBuilder::new("link", LOGO)
    .with_size(199)
    .build()
    .expect_err("Small size should fail.");

  assert!(matches!(res, Error::InputError(_)));
}

#[test]
fn size_too_big() {
  let res = QrCodeBuilder::new("link", LOGO)
    .with_size(1001)
    .build()
    .expect_err("Big size should fail.");

  assert!(matches!(res, Error::InputError(_)));
}

#[test]
fn valid() {
  let link = "https://github.com/AntoniosBarotsis/qr-rs";
  let size = 600;
  let bg_color = qr_rs_lib::Rgb([255, 0, 0]);

  let res = QrCodeBuilder::new(link, LOGO)
    .with_size(size)
    .with_bg_color(bg_color)
    .build();

  assert!(matches!(res, Ok(_)));
}
