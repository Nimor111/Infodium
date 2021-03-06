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
use infodium::models::player::Player;
use infodium::schema::games::dsl::*;

use rocket::http::{ContentType, Header, Status};

use serde_json::from_str;

#[macro_use]
mod common;
mod seed;

use common::DB_LOCK;

use seed::{gen_game, gen_league, gen_player, gen_player_game, gen_team};

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
        let game_id = gen_game(&conn, None, None).id;

        let game_count = get_all_games(&conn).len();

        let response = client
            .delete(format!("/games/{}", game_id))
            .header(Header::new("x-auth", jwt))
            .dispatch();

        let new_game_count = get_all_games(&conn).len();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(new_game_count, game_count - 1);
    })
}

#[test]
fn test_updates_a_game_successfully() {
    run_test!(|client, conn, jwt| {
        let game = gen_game(&conn, None, None);
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

#[test]
fn test_fetches_game_players_successfully() {
    run_test!(|client, conn, _jwt| {
        let game = gen_game(&conn, None, None);
        let player = gen_player(&conn, None);

        let _player_game = gen_player_game(&conn, Some(game.id), Some(player.id));

        let mut response = client
            .get(format!("/games/{}/players", game.id))
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let body = response.body_string().unwrap();
        assert_eq!(from_str::<Vec<Player>>(&body).unwrap().len(), 1);
    })
}

#[test]
fn test_fails_to_fetch_players_of_non_existent_game() {
    run_test!(|client, _conn, _jwt| {
        let response = client.get(format!("/games/{}/players", 0)).dispatch();

        assert_eq!(response.status(), Status::NotFound);
    })
}

#[test]
fn test_removes_game_on_delete_team() {
    run_test!(|client, conn, jwt| {
        let league = gen_league(&conn);
        let team = gen_team(&conn, None);

        let _game = gen_game(&conn, Some(league.id), Some(team.id));

        let game_count = get_all_games(&conn).len();

        let _response = client
            .delete(format!("/teams/{}", team.id))
            .header(Header::new("x-auth", jwt))
            .dispatch();

        let new_game_count = get_all_games(&conn).len();
        assert_eq!(new_game_count, game_count - 1);
    })
}

#[test]
fn test_removes_game_on_delete_league() {
    run_test!(|client, conn, jwt| {
        let league = gen_league(&conn);
        let team = gen_team(&conn, None);

        let _game = gen_game(&conn, Some(league.id), Some(team.id));

        let game_count = get_all_games(&conn).len();

        let _response = client
            .delete(format!("/leagues/{}", league.id))
            .header(Header::new("x-auth", jwt))
            .dispatch();

        let new_game_count = get_all_games(&conn).len();
        assert_eq!(new_game_count, game_count - 1);
    })
}
