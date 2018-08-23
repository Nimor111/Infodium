extern crate bcrypt;

use self::bcrypt::hash;

use infodium::db;
use infodium::models::league::{League, NewLeague};
use infodium::models::player::{NewPlayer, Player};
use infodium::models::team::{NewTeam, Team};
use infodium::models::user::{NewUser, User};
use infodium::schema::leagues::dsl::id as lid;
use infodium::schema::leagues::dsl::leagues;
use infodium::schema::players::dsl::id as pid;
use infodium::schema::players::dsl::players;
use infodium::schema::teams::dsl::id as tid;
use infodium::schema::teams::dsl::teams;
use infodium::schema::users::dsl::id as uid;
use infodium::schema::users::dsl::users;

use diesel;
use diesel::prelude::*;

pub fn gen_league(conn: &db::Connection) -> League {
    let new_league = NewLeague {
        name: fake!(Name.name),
        country: String::from(fake!(Lorem.word)),
        current_matchday: None,
    };

    let league_id: Vec<i32> = diesel::insert_into(leagues)
        .values(&new_league)
        .returning(lid)
        .get_results(&**conn)
        .unwrap();

    leagues
        .find(league_id[0])
        .first(&**conn)
        .expect("Failed to fetch league!")
}

pub fn gen_player(conn: &db::Connection) -> Player {
    let new_player = NewPlayer {
        name: fake!(Name.name),
        team_id: None,
        position: String::from(fake!(Lorem.word)),
        country: String::from(fake!(Lorem.word)),
        nationality: String::from(fake!(Lorem.word)),
    };

    let player_id: Vec<i32> = diesel::insert_into(players)
        .values(&new_player)
        .returning(pid)
        .get_results(&**conn)
        .unwrap();

    players
        .find(player_id[0])
        .first(&**conn)
        .expect("Failed to fetch player!")
}

pub fn gen_user(conn: &db::Connection) -> User {
    let hashed_pass = hash("password123", 6).expect("Failed to hash!");

    let new_user = NewUser {
        email: fake!(Internet.free_email),
        username: fake!(Internet.user_name),
        password: hashed_pass,
    };

    let user_id: Vec<i32> = diesel::insert_into(users)
        .values(&new_user)
        .returning(uid)
        .get_results(&**conn)
        .unwrap();

    users
        .find(user_id[0])
        .first(&**conn)
        .expect("Failed to fetch user!")
}

pub fn gen_team(conn: &db::Connection) -> Team {
    let league_id = gen_league(conn).id;

    let new_team = NewTeam {
        league_id: league_id,
        name: fake!(Name.name),
        tla: String::from(fake!(Lorem.word)),
        address: Some(fake!(Address.street_address)),
        website: None,
        facebook: None,
    };

    let team_id: Vec<i32> = diesel::insert_into(teams)
        .values(&new_team)
        .returning(tid)
        .get_results(&**conn)
        .unwrap();

    teams
        .find(team_id[0])
        .first(&**conn)
        .expect("Failed to fetch team!")
}
