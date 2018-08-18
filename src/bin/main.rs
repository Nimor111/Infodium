#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate infodium;

use infodium::rocket;

fn main() {
    rocket("dev").0.launch();
}
