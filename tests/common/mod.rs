extern crate diesel;
extern crate dotenv;
extern crate infodium;
extern crate parking_lot;
extern crate rocket;

use parking_lot::Mutex;

use rocket::local::Client;

use std::env;
use std::process::Command;

use infodium::db;
use infodium::rocket as startup;

pub static DB_LOCK: Mutex<()> = Mutex::new(());

pub fn setup() -> (Client, db::Connection) {
    dotenv::dotenv().ok();
    Command::new("dropdb")
        .arg("--if-exists")
        .arg(env::var("TEST_DATABASE_NAME").expect("Test database name not set."))
        .output()
        .expect("Failed to drop db!");
    Command::new("diesel")
        .arg("setup")
        .arg("--database-url")
        .arg(env::var("TEST_DATABASE_URL").expect("Test database url not set."))
        .output()
        .expect("Failed to setup test db");
    let _lock = DB_LOCK.lock();
    let (rocket, db) = startup("test");
    let client = Client::new(rocket).expect("Rocket client");
    #[allow(unused_variables)]
    let conn = db.get().unwrap();

    (client, db::Connection(conn))
}
