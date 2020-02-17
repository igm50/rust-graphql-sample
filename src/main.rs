mod controller;
mod entity;
mod schema;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let schema = std::sync::Arc::new(schema::create_schema());

  HttpServer::new(move || {
    App::new()
      .data(schema.clone())
      .route("/", web::get().to(controller::health))
      .route("/graphiql", web::get().to(controller::graphiql))
      .route("/graphql", web::post().to(controller::graphql))
  })
  .bind("0.0.0.0:8000")?
  .run()
  .await
}

// integration tests
#[cfg(test)]
mod tests {
  use crate::controller::*;
  use actix_web::{test, web, App};

  #[actix_rt::test]
  async fn test_health() {
    let path = "/";
    let mut app = test::init_service(App::new().route(path, web::get().to(health))).await;
    let req = test::TestRequest::get()
      .uri(path)
      .header("content-type", "text/plain")
      .to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_success());

    let req = test::TestRequest::post()
      .uri(path)
      .header("content-type", "text/plain")
      .to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_client_error());
  }
}
