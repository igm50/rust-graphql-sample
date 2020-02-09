#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate mysql;

mod controller;
mod repository;

#[get("/health")]
fn health() -> &'static str {
  "OK."
}

fn rocket() -> rocket::Rocket {
  rocket::ignite().mount("/", routes![controller::index::index, health])
}

fn main() {
  let _connection = repository::Connection::connect();

  rocket().launch();
}

#[cfg(test)]
mod test {
  use super::rocket;
  use rocket::http::Status;
  use rocket::local::Client;

  #[test]
  fn health_check() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut health = client.get("/health").dispatch();
    assert_eq!(health.status(), Status::Ok);
    assert_eq!(health.body_string(), Some("OK.".into()));
    assert_ne!(health.body_string(), Some("NG.".into()));
  }
}
