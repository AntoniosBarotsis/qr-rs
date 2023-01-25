pub mod logos;

use qr_rs_lib::Rgb;

/// Converts the passed hex string into its RGB representation.
pub fn hex_to_rgb(hex: &str) -> Option<Rgb> {
  if hex.len() != 6 {
    return None;
  }

  let x = u8::from_str_radix(&hex[0..2], 16).ok()?;
  let y = u8::from_str_radix(&hex[2..4], 16).ok()?;
  let z = u8::from_str_radix(&hex[4..6], 16).ok()?;

  Some(Rgb([x, y, z]))
}

/// Fetches an image from the passed link and returns its bytes.
///
/// The method additionally verifies that the `Content-Type` is of type `image`.
pub async fn read_image_bytes_async(link: &str) -> Option<Vec<u8>> {
  let resp = reqwest::get(link).await.ok()?;

  // TODO Test with actual links
  let content_type_is_image = resp
    .headers()
    .get("Content-Type")
    .map(reqwest::header::HeaderValue::to_str)?
    .ok()
    .map(|content_type| content_type.contains("image"))?;

  if !content_type_is_image {
    return None;
  }

  let resp_bytes = resp.bytes().await.ok()?;
  let res = resp_bytes.into_iter().collect::<Vec<_>>();

  Some(res)
}
