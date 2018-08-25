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
use infodium::models::player::Player;
use infodium::models::team::Team;
use infodium::schema::teams::dsl::*;

use rocket::http::{ContentType, Header, Status};

use serde_json::from_str;

#[macro_use]
mod common;
mod seed;

use common::DB_LOCK;

use seed::{gen_league, gen_player, gen_team};

fn get_all_teams(conn: &db::Connection) -> Vec<Team> {
    teams.load::<Team>(&**conn).expect("Error loading teams!")
}

fn fetch_team(team_id: i32, conn: &db::Connection) -> Team {
    teams
        .find(team_id)
        .first(&**conn)
        .expect("Failed to fetch team!")
}

#[test]
fn test_adds_a_team_successfully() {
    run_test!(|client, conn, jwt| {
        let team_count = get_all_teams(&conn).len();
        let league = gen_league(&conn);

        let body = json!({
            "league_id": league.id,
            "name": fake!(Name.name),
            "tla": String::from(fake!(Lorem.word)),
        }).to_string();

        let response = client
            .post("/teams")
            .header(ContentType::JSON)
            .header(Header::new("x-auth", jwt))
            .body(body)
            .dispatch();

        let new_team_count = get_all_teams(&conn).len();
        assert_eq!(response.status(), Status::Created);
        assert_eq!(new_team_count, team_count + 1);
    })
}

#[test]
fn test_deletes_a_team_successfully() {
    run_test!(|client, conn, jwt| {
        let team_id = gen_team(&conn, None).id;

        let team_count = get_all_teams(&conn).len();

        let response = client
            .delete(format!("/teams/{}", team_id))
            .header(Header::new("x-auth", jwt))
            .dispatch();

        let new_team_count = get_all_teams(&conn).len();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(new_team_count, team_count - 1);
    })
}

#[test]
fn test_updates_a_team_successfully() {
    run_test!(|client, conn, jwt| {
        let team = gen_team(&conn, None);
        let new_name = fake!(Name.name);

        let body = json!({
            "id": team.id,
            "league_id": team.league_id,
            "name": new_name.clone(),
            "tla": team.tla,
            "address": team.address,
            "website": team.website,
            "facebook": team.facebook
        }).to_string();

        let response = client
            .put(format!("/teams/{}", team.id))
            .header(ContentType::JSON)
            .header(Header::new("x-auth", jwt))
            .body(body)
            .dispatch();

        let returned_team = fetch_team(team.id, &conn);

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(returned_team.name, new_name);
    })
}

#[test]
fn test_fetches_team_players_successfully() {
    run_test!(|client, conn, _jwt| {
        let team = gen_team(&conn, None);
        let _players = (0..2)
            .map(|_| gen_player(&conn, Some(team.id)))
            .collect::<Vec<Player>>();

        let mut response = client.get(format!("/teams/{}/players", team.id)).dispatch();

        assert_eq!(response.status(), Status::Ok);

        let body = response.body_string().unwrap();
        assert_eq!(from_str::<Vec<Player>>(&body).unwrap().len(), 2);
    })
}
