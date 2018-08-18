#![feature(plugin)]
#![plugin(speculate)]

extern crate infodium;
extern crate parking_lot;
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

use diesel::prelude::*;

use rocket::http::{ContentType, Status};

use infodium::db;

mod common;

mod schema {
    table! {
        players (id) {
            id -> Int4,
            team_id -> Nullable<Int4>,
            name -> Text,
            position -> Text,
            country -> Text,
            nationality -> Text,
        }
    }
}

use schema::players;
use schema::players::dsl::*;

#[derive(Queryable, PartialEq, Debug)]
struct Player {
    id: i32,
    team_id: Option<i32>,
    name: String,
    position: String,
    country: String,
    nationality: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "players"]
struct NewPlayer {
    name: String,
    team_id: Option<i32>,
    position: String,
    country: String,
    nationality: String,
}

fn gen_player(conn: &db::Connection) -> i32 {
    let new_player = NewPlayer {
        name: fake!(Name.name),
        position: String::from(fake!(Lorem.word)),
        country: String::from(fake!(Lorem.word)),
        nationality: String::from(fake!(Lorem.word)),
        team_id: None
    };

    let player_id = diesel::insert_into(players)
        .values(&new_player)
        .returning(id)
        .get_results(&**conn)
        .unwrap();

    player_id[0]
}

fn get_all_players(conn: &db::Connection) -> Vec<Player> {
    players.load::<Player>(&**conn).expect("Error loading players!")
}

fn delete_player(player_id: i32, conn: &db::Connection) -> usize {
    diesel::delete(players::table.find(player_id)).execute(&**conn).expect("Error deleting player!")
}

speculate! {
    before {
        let (client, conn) = common::setup();
    }

    describe "player tests" {
        it "adds a player successfully" {
            let player_count = get_all_players(&conn).len();

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

            let new_player_count = get_all_players(&conn).len();
            assert_eq!(response.status(), Status::Ok);
            assert_eq!(new_player_count, player_count + 1);
        }

        it "deletes a player successfully" {
            let player_id = gen_player(&conn);

            let player_count = get_all_players(&conn).len();
            delete_player(player_id, &conn);

            let new_player_count = get_all_players(&conn).len();
            assert_eq!(new_player_count, player_count - 1);
        }
    }
}
