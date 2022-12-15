use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
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
}

#[get("/")]
#[allow(clippy::unused_async)]
async fn help() -> impl Responder {
  let msg = concat!(
    "Endpoints:\n",
    " - /qr [GET]\n",
    "   Query Params: link={string}\n",
    "   Example: /qr?link=https://github.com/AntoniosBarotsis\n"
  );

  HttpResponse::Ok().body(msg)
}

#[get("qr")]
#[allow(clippy::unused_async)]
async fn qr(link: web::Query<Input>) -> impl Responder {
  let input = link.into_inner();

  match qr_rs::generate_qr_code(input.link.as_str()) {
    Ok(body) => HttpResponse::Ok().body(body),
    Err(e) => HttpResponse::BadRequest().body(e.to_string()),
  }
}
