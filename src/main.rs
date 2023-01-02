use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use qr_rs_lib::{QrCodeBuilder, Rgba, DEFAULT_SIZE};
use serde::Deserialize;

static PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Starting server on port {PORT}");

  HttpServer::new(|| App::new().service(qr).service(help))
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}

#[derive(Debug, Deserialize)]
struct Input {
  link: String,
  size: Option<u32>,
  bg_color: Option<String>,
}

fn hex_to_rgba(hex: &str) -> Option<Rgba<u8>> {
  if hex.len() != 6 {
    return None;
  }

  let x = u8::from_str_radix(&hex[0..2], 16).ok()?;
  let y = u8::from_str_radix(&hex[2..4], 16).ok()?;
  let z = u8::from_str_radix(&hex[4..6], 16).ok()?;

  Some(Rgba([x, y, z, 255]))
}

#[get("/")]
#[allow(clippy::unused_async)]
async fn help() -> impl Responder {
  let msg = concat!(
    "Endpoints:\n",
    " - /qr [GET]\n",
    "   Query Params: link={string}, size={number}, bg_color={hex}\n",
    "   Example: /qr?link=https://github.com/AntoniosBarotsis\n"
  );

  HttpResponse::Ok().body(msg)
}

#[get("qr")]
#[allow(clippy::unused_async)]
async fn qr(link: web::Query<Input>) -> impl Responder {
  let input = link.into_inner();

  let bg_color = input.bg_color.and_then(|s| hex_to_rgba(&s));

  let builder = QrCodeBuilder::new(input.link.as_str())
    .with_size(input.size.unwrap_or(DEFAULT_SIZE))
    .with_bg_color(bg_color)
    .build();

  match builder {
    Ok(body) => HttpResponse::Ok().content_type("image/png").body(body),
    Err(e) => HttpResponse::BadRequest().body(e.to_string()),
  }
}
