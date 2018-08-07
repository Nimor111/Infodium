#![feature(attr_literals)]
#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] pub extern crate serde_derive;
#[macro_use] pub extern crate rocket_contrib;
#[macro_use] pub extern crate diesel;
#[macro_use] pub extern crate dotenv_codegen;
extern crate chrono;

pub extern crate r2d2;
pub extern crate rocket;
pub extern crate r2d2_diesel;

pub mod models;
pub mod routes;
pub mod schema;
pub mod db;
