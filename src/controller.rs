use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

use crate::schema::Schema;

pub async fn health() -> HttpResponse {
  HttpResponse::Ok().content_type("plain/text").body("Ok.")
}

pub async fn graphiql() -> HttpResponse {
  let html = graphiql_source("/graphql");
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}

pub async fn graphql(
  st: web::Data<Arc<Schema>>,
  data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
  let user = web::block(move || {
    let res = data.execute(&st, &());
    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
  })
  .await?;
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(user),
  )
}

#[cfg(test)]
mod test {
  use super::*;

  #[actix_rt::test]
  async fn health_check() {
    let res = health().await;
    assert_eq!(res.status(), http::StatusCode::OK);
    assert_eq!(
      res.body().as_ref().unwrap(),
      &actix_web::dev::Body::from("Ok.")
    );
  }

  #[actix_rt::test]
  async fn graphiql_res() {
    let res = graphiql().await;
    assert_eq!(res.status(), http::StatusCode::OK);
  }
}
