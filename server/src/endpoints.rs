use actix_web::{get, web, HttpResponse, Responder};
use common::{hex_to_rgb, logos::Logo, read_image_bytes_async};
use qr_rs_lib::{QrCodeBuilder, DEFAULT_SIZE};
use serde::Deserialize;

use crate::error::ServerError;

static WHITE_HEX: &str = "FFFFFF";

#[get("/")]
#[allow(clippy::unused_async)]
pub async fn index() -> impl Responder {
  HttpResponse::PermanentRedirect()
    .customize()
    .insert_header(("Location", "/help"))
}

#[get("/help")]
#[allow(clippy::unused_async)]
pub async fn help() -> impl Responder {
  let msg = concat!(
    "Endpoints:\n",
    " - /qr [GET]\n",
    "   Query Params:\n",
    "       content:         string\n",
    "       size:            number    [optional]\n",
    "       bg_color:        hex       [optional]\n",
    "       logo:            string    [optional]\n",
    "       logo_web_source: string    [optional]\n\n",
    "   Example: /qr?content=https://github.com/AntoniosBarotsis\n\n",
    "   - content:         The text the qr code should contain.\n",
    "   - size:            The size of the QR Code                      [default: 600]\n",
    "   - bg_color:        The background color of the QR Code (in hex) [default: FFFFFF]\n",
    "   - logo:            The name of the logo to use in the overlay.  [default: google]\n",
    "   - logo_web_source: URL to the logo (must be a valid PNG/JPEG)   [default: None]"
  );

  HttpResponse::Ok().body(msg)
}

#[get("qr")]
#[allow(clippy::unused_async)]
pub async fn qr(content: web::Query<Input>) -> Result<HttpResponse, ServerError> {
  let input = content.into_inner();

  let bg_color = input
    .bg_color
    .or_else(|| Some(WHITE_HEX.to_owned()))
    .and_then(|s| hex_to_rgb(&s))
    .ok_or(ServerError::InvalidColor)?;

  let logo = match input.logo_web_source {
    Some(l) => read_image_bytes_async(&l).await,
    None => Some(Logo::try_from(input.logo)?.into()),
  }
  .ok_or_else(|| ServerError::InvalidLogo("Error reading logo".to_owned()))?;

  let qr_code = QrCodeBuilder::new(input.content.as_str(), &logo)
    .with_size(input.size.unwrap_or(DEFAULT_SIZE))
    .with_bg_color(bg_color)
    .build()?;

  Ok(HttpResponse::Ok().content_type("image/png").body(qr_code))
}

#[derive(Debug, Deserialize)]
pub struct Input {
  content: String,
  size: Option<u32>,
  bg_color: Option<String>,
  logo: Option<String>,
  logo_web_source: Option<String>,
}
