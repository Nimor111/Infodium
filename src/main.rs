#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate infodium;

use infodium::rocket;
use infodium::rocket_contrib::*;
use infodium::rocket_contrib::{Json, Value};

#[get("/")]
fn index() -> Json<Value> {
    Json(json!({"greeting": "Hello world!"}))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
