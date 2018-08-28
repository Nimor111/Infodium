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
use infodium::models::league::League;
use infodium::models::team::Team;
use infodium::schema::leagues::dsl::*;

use rocket::http::{ContentType, Header, Status};

use serde_json::from_str;

#[macro_use]
mod common;
mod seed;

use common::DB_LOCK;
use seed::{gen_league, gen_team};

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
    run_test!(|client, conn, jwt| {
        let league_count = get_all_leagues(&conn).len();

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
        assert_eq!(response.status(), Status::Created);
        assert_eq!(new_league_count, league_count + 1);
    })
}

#[test]
fn test_fetches_a_league_successfully() {
    run_test!(|client, conn, _jwt| {
        let league = gen_league(&conn);

        let mut response = client.get(format!("/leagues/{}", league.id)).dispatch();

        assert_eq!(response.status(), Status::Ok);

        let body = response.body_string().unwrap();
        let received_league = from_str::<League>(&body).unwrap();
        assert_eq!(received_league.name, league.name);
        assert_eq!(received_league.country, league.country);
        assert_eq!(received_league.current_matchday, league.current_matchday);
    })
}

#[test]
fn test_deletes_a_league_successfully() {
    run_test!(|client, conn, jwt| {
        let league_id = gen_league(&conn).id;

        let league_count = get_all_leagues(&conn).len();

        let response = client
            .delete(format!("/leagues/{}", league_id))
            .header(Header::new("x-auth", jwt))
            .dispatch();

        let new_league_count = get_all_leagues(&conn).len();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(new_league_count, league_count - 1);
    })
}

#[test]
fn test_updates_a_league_successfully() {
    run_test!(|client, conn, jwt| {
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
            .header(Header::new("x-auth", jwt))
            .body(body)
            .dispatch();

        let returned_league = fetch_league(league.id, &conn);

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(returned_league.name, new_name);
    })
}

#[test]
fn test_fetches_league_teams_successfully() {
    run_test!(|client, conn, _jwt| {
        let league = gen_league(&conn);
        let _teams = (0..2)
            .map(|_| gen_team(&conn, Some(league.id)))
            .collect::<Vec<Team>>();

        let mut response = client
            .get(format!("/leagues/{}/teams", league.id))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let body = response.body_string().unwrap();
        assert_eq!(from_str::<Vec<Team>>(&body).unwrap().len(), 2);
    })
}

#[test]
fn test_returns_not_found_if_league_does_not_exist() {
    run_test!(|client, _conn, _jwt| {
        let mut response = client.get(format!("/leagues/{}/teams", 0)).dispatch();

        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(
            response.body_string(),
            Some("\"Resource not found!\"".to_string())
        );
    })
}
