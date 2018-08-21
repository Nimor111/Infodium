extern crate infodium;
extern crate rocket;
#[macro_use]
extern crate fake;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate validator;

use self::rocket::http::{ContentType, Status};

use self::validator::Validate;

#[macro_use]
mod common;

use common::DB_LOCK;

use self::infodium::schema::users;
use infodium::schema::users::dsl::*;
