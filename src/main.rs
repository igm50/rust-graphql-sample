mod controller;
mod entity;
mod repository;
mod schema;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;

fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/", web::get().to(controller::health))
    .route("/graphiql", web::get().to(controller::graphiql))
    .route("/graphql", web::post().to(controller::graphql));
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let repository = Arc::new(repository::Connection::new());
  if let Err(e) = repository.init_table() {
    panic!(format!("{}", e))
  }

  let schema = Arc::new(schema::create_schema(repository.clone()));

  HttpServer::new(move || App::new().data(schema.clone()).configure(config))
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

// integration tests
#[cfg(test)]
mod tests {
  use super::config;
  use actix_web::{test, App};

  #[actix_rt::test]
  async fn test_health() {
    let mut app = test::init_service(App::new().configure(config)).await;
    let req = test::TestRequest::get()
      .uri("/")
      .header("content-type", "text/plain")
      .to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_success());

    let req = test::TestRequest::post()
      .uri("/")
      .header("content-type", "text/plain")
      .to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_client_error());
  }
}
