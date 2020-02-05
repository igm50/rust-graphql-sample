#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod controller;

fn main() {
    rocket::ignite().mount("/", routes![controller::index::index]).launch();
}
