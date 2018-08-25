#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate infodium;

use infodium::logger;
use infodium::rocket;

fn main() {
    logger::setup_logger().unwrap();

    rocket().0.launch();
}
