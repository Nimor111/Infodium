#[macro_use]
extern crate rocket_contrib;
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
pub extern crate fake;
extern crate diesel;
extern crate infodium;
extern crate rocket;

use diesel::prelude::*;

use infodium::db;
use infodium::models::league::{League, NewLeague};
use infodium::schema::leagues::dsl::*;
use infodium::utils::util::generate_jwt_token;

use rocket::http::{ContentType, Header, Status};

#[macro_use]
mod common;
mod seed;

use common::DB_LOCK;
use seed::gen_league;

fn get_all_leagues(conn: &db::Connection) -> Vec<League> {
    leagues
        .load::<League>(&**conn)
        .expect("Error loading leagues!")
}

fn fetch_league(league_id: i32, conn: &db::Connection) -> League {
    leagues
        .find(league_id)
        .first(&**conn)
        .expect("Failed to fetch league!")
}

#[test]
fn test_adds_a_league_successfully() {
    run_test!(|client, conn| {
        let league_count = get_all_leagues(&conn).len();
        let jwt = generate_jwt_token(json!({"id": 1})).expect("Failed to generate jwt!");

        let body = json!({
            "name": fake!(Name.name),
            "country": String::from(fake!(Lorem.word)),
        }).to_string();

        let response = client
            .post("/leagues")
            .header(ContentType::JSON)
            .header(Header::new("x-auth", jwt))
            .body(body)
            .dispatch();

        let new_league_count = get_all_leagues(&conn).len();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(new_league_count, league_count + 1);
    })
}

#[test]
fn test_deletes_a_league_successfully() {
    run_test!(|client, conn| {
        let league_id = gen_league(&conn).id;

        let league_count = get_all_leagues(&conn).len();

        let response = client.delete(format!("/leagues/{}", league_id)).dispatch();

        let new_league_count = get_all_leagues(&conn).len();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(new_league_count, league_count - 1);
    })
}

#[test]
fn test_updates_a_league_successfully() {
    run_test!(|client, conn| {
        let league = gen_league(&conn);
        let new_name = fake!(Name.name);

        let body = json!({
            "id": league.id,
            "name": new_name.clone(),
            "country": league.country,
            "current_matchday": league.current_matchday
        }).to_string();

        let response = client
            .put(format!("/leagues/{}", league.id))
            .header(ContentType::JSON)
            .body(body)
            .dispatch();

        let returned_league = fetch_league(league.id, &conn);

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(returned_league.name, new_name);
    })
}
