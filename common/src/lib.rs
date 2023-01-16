pub mod logos;

use qr_rs_lib::Rgb;

pub fn hex_to_rgb(hex: &str) -> Option<Rgb> {
  if hex.len() != 6 {
    return None;
  }

  let x = u8::from_str_radix(&hex[0..2], 16).ok()?;
  let y = u8::from_str_radix(&hex[2..4], 16).ok()?;
  let z = u8::from_str_radix(&hex[4..6], 16).ok()?;

  Some(Rgb([x, y, z]))
}
