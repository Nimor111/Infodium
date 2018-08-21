extern crate diesel;
extern crate dotenv;
extern crate infodium;
extern crate parking_lot;
extern crate rocket;

use self::parking_lot::Mutex;

use rocket::local::Client;

use std::env;
use std::process::Command;

use infodium::db;
pub use infodium::rocket as startup;

pub static DB_LOCK: Mutex<()> = Mutex::new(());

#[macro_export]
macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => {{
        let _lock = DB_LOCK.lock();
        let (rocket, db) = startup("test");
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db::Connection(
            db.get()
                .expect("failed to get database connection for testing"),
        );
        $block
    }};
}
