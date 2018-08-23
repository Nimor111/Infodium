extern crate parking_lot;

use self::parking_lot::Mutex;

pub static DB_LOCK: Mutex<()> = Mutex::new(());

#[macro_export]
macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => {{
        use infodium::db;
        use infodium::rocket as startup;
        use rocket::local::Client;

        let _lock = DB_LOCK.lock();
        let (rocket, db) = startup();
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db::Connection(
            db.get()
                .expect("failed to get database connection for testing"),
        );
        $block
    }};
}
