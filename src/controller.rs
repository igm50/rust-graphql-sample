use actix_web::HttpResponse;
use juniper::http::graphiql::graphiql_source;

pub async fn health() -> HttpResponse {
  HttpResponse::Ok().content_type("plain/text").body("Ok.")
}

pub async fn graphiql() -> HttpResponse {
  let html = graphiql_source("http://127.0.0.1:8000/graphql");
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
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
  async fn graphiql_res() {
    let res = graphiql().await;
    assert_eq!(res.status(), http::StatusCode::OK);
  }
}
