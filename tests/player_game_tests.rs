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
use infodium::models::player_game::PlayerGame;
use infodium::schema::player_games::dsl::*;

use rocket::http::{ContentType, Header, Status};

#[macro_use]
mod common;
mod seed;

use common::DB_LOCK;

use seed::{gen_game, gen_player, gen_player_game};

fn get_all_player_games(conn: &db::Connection) -> Vec<PlayerGame> {
    player_games
        .load::<PlayerGame>(&**conn)
        .expect("Error loading player games!")
}

#[test]
fn test_adds_a_player_game_successfully() {
    run_test!(|client, conn, jwt| {
        let player_game_count = get_all_player_games(&conn).len();

        let player = gen_player(&conn, None);
        let game = gen_game(&conn, None, None);

        let body = json!({
            "game_id": game.id,
            "player_id": player.id
        }).to_string();

        let response = client
            .post("/player-games")
            .header(ContentType::JSON)
            .header(Header::new("x-auth", jwt))
            .body(body)
            .dispatch();

        assert_eq!(response.status(), Status::Created);

        let new_player_game_count = get_all_player_games(&conn).len();
        assert_eq!(new_player_game_count, player_game_count + 1);
    })
}

#[test]
fn test_removes_player_game_on_delete_game() {
    run_test!(|client, conn, jwt| {
        let player = gen_player(&conn, None);
        let game = gen_game(&conn, None, None);

        let _player_game = gen_player_game(&conn, Some(game.id), Some(player.id));

        let player_game_count = get_all_player_games(&conn).len();

        let _response = client
            .delete(format!("/games/{}", game.id))
            .header(Header::new("x-auth", jwt))
            .dispatch();

        let new_player_game_count = get_all_player_games(&conn).len();
        assert_eq!(new_player_game_count, player_game_count - 1);
    })
}

#[test]
fn test_removes_player_game_on_delete_player() {
    run_test!(|client, conn, jwt| {
        let player = gen_player(&conn, None);
        let game = gen_game(&conn, None, None);

        let _player_game = gen_player_game(&conn, Some(game.id), Some(player.id));

        let player_game_count = get_all_player_games(&conn).len();

        let _response = client
            .delete(format!("/players/{}", player.id))
            .header(Header::new("x-auth", jwt))
            .dispatch();

        let new_player_game_count = get_all_player_games(&conn).len();
        assert_eq!(new_player_game_count, player_game_count - 1);
    })
}
