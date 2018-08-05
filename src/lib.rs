#[macro_use] pub extern crate serde_derive;
#[macro_use] pub extern crate rocket_contrib;
#[macro_use] pub extern crate diesel;

pub extern crate r2d2;
pub extern crate rocket;
pub extern crate r2d2_diesel;

pub mod models;
pub mod schema;
pub mod db;
