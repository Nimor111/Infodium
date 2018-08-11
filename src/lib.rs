#![feature(attr_literals)]
#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] pub extern crate serde_derive;
#[macro_use] pub extern crate rocket_contrib;
#[macro_use] pub extern crate diesel;
#[macro_use] pub extern crate dotenv_codegen;
#[macro_use] pub extern crate serde_json;
pub extern crate frank_jwt;
extern crate uuid;
extern crate chrono;

pub extern crate r2d2;
pub extern crate rocket;
pub extern crate r2d2_diesel;

pub mod db;
pub mod schema;

pub mod models;
pub mod routes;
pub mod guards;
pub mod utils;
