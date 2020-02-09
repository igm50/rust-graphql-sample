use actix_web::{HttpRequest, HttpResponse};

pub async fn hello() -> HttpResponse {
  HttpResponse::Ok()
    .content_type("plain/text")
    .body("Hello world!")
}

pub async fn health() -> HttpResponse {
  HttpResponse::Ok().content_type("plain/text").body("Ok.")
}

#[cfg(test)]
mod test {
  use super::*;

  #[actix_rt::test]
  async fn health_check() {
    let res = health().await;
    assert_eq!(res.status(), http::StatusCode::OK);
  }

  #[actix_rt::test]
  async fn index() {
    let res = health().await;
    assert_eq!(res.status(), http::StatusCode::OK);
  }
}
