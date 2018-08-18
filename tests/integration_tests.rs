#![feature(plugin)]
#![plugin(speculate)]

extern crate infodium;
extern crate parking_lot;
extern crate rocket;
#[macro_use]
extern crate fake;
#[macro_use]
extern crate rocket_contrib;

use parking_lot::Mutex;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

use infodium::models::player::Player;
use infodium::rocket as startup;

static DB_LOCK: Mutex<()> = Mutex::new(());

speculate! {
    before {
        let _lock = DB_LOCK.lock();
        let (rocket, db) = startup("test");
        let client = Client::new(rocket).expect("Rocket client");
        #[allow(unused_variables)]
        let conn = &*db.get().expect("Failed to get db connection for testing!");
    }

    describe "player tests" {
        it "adds a player successfully" {
            let players = Player::all(conn);

            let body = json!({
                "name": fake!(Name.name),
                "position": String::from(fake!(Lorem.word)),
                "country": String::from(fake!(Lorem.word)),
                "nationality": String::from(fake!(Lorem.word)),
            }).to_string();

            let response = client
                .post("/players")
                .header(ContentType::JSON)
                .body(body)
                .dispatch();

            let new_players = Player::all(conn);
            assert_eq!(response.status(), Status::Ok);
            assert_eq!(new_players.len(), players.len() + 1);
        }
    }
}
