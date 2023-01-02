use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use qr_rs_lib::{QrCodeBuilder, DEFAULT_SIZE};
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
pub struct Input {
  pub link: String,
  size: Option<u32>,
}

#[get("/")]
#[allow(clippy::unused_async)]
async fn help() -> impl Responder {
  let msg = concat!(
    "Endpoints:\n",
    " - /qr [GET]\n",
    "   Query Params: link={string}, size={number}\n",
    "   Example: /qr?link=https://github.com/AntoniosBarotsis\n"
  );

  HttpResponse::Ok().body(msg)
}

#[get("qr")]
#[allow(clippy::unused_async)]
async fn qr(link: web::Query<Input>) -> impl Responder {
  let input = link.into_inner();

  let tmp = QrCodeBuilder::new(input.link.as_str())
    .with_size(input.size.unwrap_or(DEFAULT_SIZE))
    .build();

  match tmp {
    Ok(body) => HttpResponse::Ok().body(body),
    Err(e) => HttpResponse::BadRequest().body(e.to_string()),
  }
}
