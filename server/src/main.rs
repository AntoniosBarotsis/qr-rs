pub mod endpoints;
mod error;
mod logos;
use endpoints::{help, qr};

use actix_web::{App, HttpServer};

use crate::endpoints::index;

static PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Starting server on port {PORT}");

  HttpServer::new(|| App::new().service(index).service(qr).service(help))
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
  use actix_web::{body::MessageBody, http::StatusCode, test, App};
  use pretty_assertions::assert_eq;

  use crate::endpoints::{help, index, qr};

  #[actix_web::test]
  async fn test_get_index() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::PERMANENT_REDIRECT);

    let location = resp.headers().get("Location");
    assert!(location.is_some());

    let tmp = location.unwrap().to_str();
    assert!(tmp.is_ok());
    assert_eq!(tmp.unwrap(), "/help");
  }

  #[actix_web::test]
  async fn test_get_help() {
    let app = test::init_service(App::new().service(help)).await;
    let req = test::TestRequest::get().uri("/help").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
  }

  #[actix_web::test]
  async fn test_get_qr_no_content() {
    let app = test::init_service(App::new().service(qr)).await;
    let req = test::TestRequest::get().uri("/qr").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
  }

  #[actix_web::test]
  async fn test_get_qr_empty_content() {
    let app = test::init_service(App::new().service(qr)).await;
    let req = test::TestRequest::get().uri("/qr?content=").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
  }

  #[actix_web::test]
  async fn test_get_qr_with_content() {
    let app = test::init_service(App::new().service(qr)).await;
    let req = test::TestRequest::get()
      .uri("/qr?content=https://github.com/AntoniosBarotsis/qr-rs")
      .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
  }

  #[actix_web::test]
  async fn test_get_qr_with_small_size() {
    let app = test::init_service(App::new().service(qr)).await;
    let req = test::TestRequest::get()
      .uri("/qr?content=https://github.com/AntoniosBarotsis/qr-rs&size=199")
      .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let body = resp.into_body().try_into_bytes();
    assert!(body.is_ok());

    let body = body.unwrap();
    assert_eq!(body, "\"Size should be between 200 and 1000.\"");
  }

  #[actix_web::test]
  async fn test_get_qr_with_big_size() {
    let app = test::init_service(App::new().service(qr)).await;
    let req = test::TestRequest::get()
      .uri("/qr?content=https://github.com/AntoniosBarotsis/qr-rs&size=1001")
      .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let body = resp.into_body().try_into_bytes();
    assert!(body.is_ok());

    let body = body.unwrap();
    assert_eq!(body, "\"Size should be between 200 and 1000.\"");
  }

  #[actix_web::test]
  async fn test_get_qr_with_ok_size() {
    let app = test::init_service(App::new().service(qr)).await;
    let req = test::TestRequest::get()
      .uri("/qr?content=https://github.com/AntoniosBarotsis/qr-rs&size=600")
      .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
  }

  #[actix_web::test]
  async fn test_get_qr_with_invalid_color() {
    let app = test::init_service(App::new().service(qr)).await;
    let req = test::TestRequest::get()
      .uri("/qr?content=https://github.com/AntoniosBarotsis/qr-rs&bg_color=GGGGGG")
      .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let body = resp.into_body().try_into_bytes();
    assert!(body.is_ok());

    let body = body.unwrap();
    assert_eq!(body, "\"Invalid color\"");
  }

  #[actix_web::test]
  async fn test_get_qr_with_invalid_color_length() {
    let app = test::init_service(App::new().service(qr)).await;
    let req = test::TestRequest::get()
      .uri("/qr?content=https://github.com/AntoniosBarotsis/qr-rs&bg_color=GGGGG")
      .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let body = resp.into_body().try_into_bytes();
    assert!(body.is_ok());

    let body = body.unwrap();
    assert_eq!(body, "\"Invalid color\"");
  }

  #[actix_web::test]
  async fn test_get_qr_with_size_color() {
    let app = test::init_service(App::new().service(qr)).await;
    let req = test::TestRequest::get()
      .uri("/qr?content=https://github.com/AntoniosBarotsis/qr-rs&bg_color=FFFFFF")
      .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
  }

  #[actix_web::test]
  async fn test_get_qr_with_google_logo() {
    let app = test::init_service(App::new().service(qr)).await;
    let req = test::TestRequest::get()
      .uri("/qr?content=https://github.com/AntoniosBarotsis/qr-rs&logo=GooGlE")
      .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
  }

  #[actix_web::test]
  async fn test_get_qr_with_invalid_logo() {
    let app = test::init_service(App::new().service(qr)).await;
    let req = test::TestRequest::get()
      .uri("/qr?content=https://github.com/AntoniosBarotsis/qr-rs&logo=invalid")
      .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
  }
}
