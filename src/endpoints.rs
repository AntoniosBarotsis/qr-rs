use actix_web::{get, web, HttpResponse, Responder};
use qr_rs_lib::{QrCodeBuilder, Rgb, DEFAULT_SIZE};
use serde::Deserialize;

use crate::{error::Error, logos::Logo};

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
    "   Query Params: content={string}, size={number}, bg_color={hex}\n",
    "   Example: /qr?content=https://github.com/AntoniosBarotsis\n"
  );

  HttpResponse::Ok().body(msg)
}

#[get("qr")]
#[allow(clippy::unused_async)]
pub async fn qr(content: web::Query<Input>) -> Result<HttpResponse, Error> {
  let input = content.into_inner();

  let bg_color = input
    .bg_color
    .or_else(|| Some(WHITE_HEX.to_owned()))
    .and_then(|s| hex_to_rgb(&s))
    .ok_or(Error::InvalidColor)?;

  let logo = Logo::try_from(input.logo)?.into();

  let qr_code = QrCodeBuilder::new(input.content.as_str(), logo)
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
}

fn hex_to_rgb(hex: &str) -> Option<Rgb> {
  if hex.len() != 6 {
    return None;
  }

  let x = u8::from_str_radix(&hex[0..2], 16).ok()?;
  let y = u8::from_str_radix(&hex[2..4], 16).ok()?;
  let z = u8::from_str_radix(&hex[4..6], 16).ok()?;

  Some(Rgb([x, y, z]))
}
