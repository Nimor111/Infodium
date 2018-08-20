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
extern crate dotenv;

use std::env;
use std::process::Command;

use dotenv::dotenv;

use diesel::prelude::*;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

use infodium::db;

#[macro_use]
mod common;

use common::startup;
use common::DB_LOCK;

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

fn gen_player(conn: &db::Connection) -> Player {
    let new_player = NewPlayer {
        name: fake!(Name.name),
        team_id: None,
        position: String::from(fake!(Lorem.word)),
        country: String::from(fake!(Lorem.word)),
        nationality: String::from(fake!(Lorem.word)),
    };

    let player_id: Vec<i32> = diesel::insert_into(players)
        .values(&new_player)
        .returning(id)
        .get_results(&**conn)
        .unwrap();

    players::table
        .find(player_id[0])
        .first(&**conn)
        .expect("Failed to fetch player!")
}

fn get_all_players(conn: &db::Connection) -> Vec<Player> {
    players
        .load::<Player>(&**conn)
        .expect("Error loading players!")
}

fn delete_player(player_id: i32, conn: &db::Connection) -> usize {
    diesel::delete(players::table.find(player_id))
        .execute(&**conn)
        .expect("Error deleting player!")
}

fn fetch_player(player_id: i32, conn: &db::Connection) -> Player {
    players::table
        .find(player_id)
        .first(&**conn)
        .expect("Failed to fetch player!")
}

#[test]
fn test_adds_a_player_successfully() {
    run_test!(|client, conn| {
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
    })
}

#[test]
fn test_deletes_a_player_successfully() {
    run_test!(|client, conn| {
        let player_id = gen_player(&conn).id;

        let player_count = get_all_players(&conn).len();

        let response = client.delete(format!("/players/{}", player_id)).dispatch();

        let new_player_count = get_all_players(&conn).len();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(new_player_count, player_count - 1);
    })
}

#[test]
fn test_updates_a_player_successfully() {
    run_test!(|client, conn| {
        let player = gen_player(&conn);
        let new_name = fake!(Name.name);

        let body = json!({
            "id": player.id,
            "name": new_name.clone(),
            "position": player.position,
            "country": player.country,
            "nationality": player.nationality,
        }).to_string();

        let response = client
            .put(format!("/players/{}", player.id))
            .header(ContentType::JSON)
            .body(body)
            .dispatch();
        let returned_player = fetch_player(player.id, &conn);

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(returned_player.name, new_name);
    })
}
