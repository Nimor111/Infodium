extern crate diesel;
extern crate infodium;
extern crate parking_lot;
extern crate rocket;

use self::parking_lot::Mutex;

pub static DB_LOCK: Mutex<()> = Mutex::new(());

#[macro_export]
macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => {{
        use self::infodium::db;
        use self::infodium::rocket as startup;
        use self::rocket::local::Client;

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
