#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate mysql;

mod controller;
mod repository;

fn main() {
    let _connection = repository::Connection::connect();

    rocket::ignite()
        .mount("/", routes![controller::index::index])
        .launch();
}
