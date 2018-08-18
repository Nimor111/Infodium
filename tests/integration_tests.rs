extern crate infodium;
extern crate parking_lot;
extern crate rocket;
#[macro_use]
extern crate fake;

use parking_lot::Mutex;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

use infodium::models::player::Player;
use infodium::rocket as startup;

static DB_LOCK: Mutex<()> = Mutex::new(());

// Thank you rocket examples
#[macro_export]
macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => {{
        let _lock = DB_LOCK.lock();
        let (rocket, db) = startup("test");
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db.get().expect("Failed to get db connection for testing!");
        $block
    }};
}

#[test]
fn test_player_insert() {
    run_test!(|client, conn| {
        let players = Player::all(&*conn);

        let body = r#"{
            "name": "Ivan Rakitic",
            "position": "Midfielder",
            "country": "Croatia",
            "nationality": "croatian"
        }"#;

        let response = client
            .post("/players")
            .header(ContentType::JSON)
            .body(body)
            .dispatch();

        let new_players = Player::all(&*conn);
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(new_players.len(), players.len() + 1);
    })
}
