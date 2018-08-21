#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate infodium;

use infodium::rocket;

fn main() {
    rocket().0.launch();
}
