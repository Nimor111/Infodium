#![feature(plugin)]
#![plugin(speculate)]

extern crate infodium;
extern crate parking_lot;
extern crate rocket;
#[macro_use]
extern crate fake;
#[macro_use]
extern crate rocket_contrib;

use rocket::http::{ContentType, Status};

use infodium::models::player::Player;

mod common;

speculate! {
    before {
        let (client, conn) = common::setup();
    }

    describe "player tests" {
        it "adds a player successfully" {
            let players = Player::all(&conn);

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

            let new_players = Player::all(&conn);
            assert_eq!(response.status(), Status::Ok);
            assert_eq!(new_players.len(), players.len() + 1);
        }
    }
}
