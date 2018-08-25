#[macro_use]
extern crate rocket_contrib;
extern crate diesel;
extern crate infodium;
extern crate rocket;
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate fake;

use diesel::prelude::*;

use infodium::db;
use infodium::models::game::Game;
use infodium::schema::games::dsl::*;

use rocket::http::{ContentType, Header, Status};

#[macro_use]
mod common;
mod seed;

use common::DB_LOCK;

use seed::{gen_game, gen_league, gen_team};

fn get_all_games(conn: &db::Connection) -> Vec<Game> {
    games.load::<Game>(&**conn).expect("Error loading games!")
}

fn fetch_game(game_id: i32, conn: &db::Connection) -> Game {
    games
        .find(game_id)
        .first(&**conn)
        .expect("Failed to fetch game!")
}

#[test]
fn test_adds_a_game_successfully() {
    run_test!(|client, conn, jwt| {
        let team = gen_team(&conn, None);
        let league = gen_league(&conn);
        let game_count = get_all_games(&conn).len();

        let body = json!({
            "team_id": team.id,
            "league_id": league.id,
            "venue": String::from(fake!(Lorem.word)),
            }).to_string();

        let response = client
            .post("/games")
            .header(ContentType::JSON)
            .header(Header::new("x-auth", jwt))
            .body(body)
            .dispatch();

        let new_game_count = get_all_games(&conn).len();
        assert_eq!(response.status(), Status::Created);
        assert_eq!(new_game_count, game_count + 1);
    })
}

#[test]
fn test_deletes_a_game_successfully() {
    run_test!(|client, conn, jwt| {
        let game_id = gen_game(&conn).id;

        let game_count = get_all_games(&conn).len();

        let response = client
            .delete(format!("/games/{}", game_id))
            .header(Header::new("x-auth", jwt))
            .dispatch();

        let new_game_count = get_all_games(&conn).len();

        assert_eq!(response.status(), Status::NoContent);
        assert_eq!(new_game_count, game_count - 1);
    })
}

#[test]
fn test_updates_a_game_successfully() {
    run_test!(|client, conn, jwt| {
        let game = gen_game(&conn);
        let new_venue = fake!(Name.name);

        let body = json!({
            "team_id": game.team_id,
            "league_id": game.league_id,
            "venue": new_venue.clone()
        }).to_string();

        let response = client
            .put(format!("/games/{}", game.id))
            .header(ContentType::JSON)
            .header(Header::new("x-auth", jwt))
            .body(body)
            .dispatch();
        let returned_game = fetch_game(game.id, &conn);

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(returned_game.venue, new_venue);
    })
}
