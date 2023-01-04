use qr_rs_lib::{error::Error, generate_qr_code, DEFAULT_SIZE};

#[test]
fn empty_link() {
  let res = generate_qr_code("", DEFAULT_SIZE, None).expect_err("Empty link should fail.");
  assert!(matches!(res, Error::InputError(_)));
}

#[test]
fn size_too_small() {
  let res = generate_qr_code("link", 190, None).expect_err("Small size should fail.");
  assert!(matches!(res, Error::InputError(_)));
}

#[test]
fn size_too_big() {
  let res = generate_qr_code("link", 2000, None).expect_err("Big size should fail.");
  assert!(matches!(res, Error::InputError(_)));
}

#[test]
fn valid() {
  let link = "https://github.com/AntoniosBarotsis/qr-rs";
  let size = 600;
  let bg_color = qr_rs_lib::Rgb([255, 0, 0]);

  let res = generate_qr_code(link, size, Some(bg_color));
  assert!(matches!(res, Ok(_)));
}
