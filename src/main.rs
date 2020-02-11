extern crate http;

mod controller;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(controller::hello))
      .route("/health", web::get().to(controller::health))
  })
  .bind("0.0.0.0:8000")?
  .run()
  .await
}
