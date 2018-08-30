#![allow(dead_code)]

extern crate bcrypt;
extern crate uuid;

use self::uuid::Uuid;

use self::bcrypt::hash;

use infodium::db;
use infodium::models::game::{Game, NewGame};
use infodium::models::league::{League, NewLeague};
use infodium::models::player::{NewPlayer, Player};
use infodium::models::player_game::{NewPlayerGame, PlayerGame};
use infodium::models::team::{NewTeam, Team};
use infodium::models::user::{NewUser, User};
use infodium::schema::games::dsl::games;
use infodium::schema::games::dsl::id as gid;
use infodium::schema::leagues::dsl::id as lid;
use infodium::schema::leagues::dsl::leagues;
use infodium::schema::player_games::dsl::id as pgid;
use infodium::schema::player_games::dsl::player_games;
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

pub fn gen_player(conn: &db::Connection, team_id: Option<i32>) -> Player {
    let new_player = NewPlayer {
        name: fake!(Name.name),
        team_id: team_id,
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
        username: Some(fake!(Internet.user_name)),
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

pub fn gen_team(conn: &db::Connection, league_id: Option<i32>) -> Team {
    let league_id = match league_id {
        Some(i) => i,
        None => gen_league(conn).id,
    };

    let new_team = NewTeam {
        league_id: Some(league_id),
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

pub fn gen_game(conn: &db::Connection, league_id: Option<i32>, team_id: Option<i32>) -> Game {
    let league_id = match league_id {
        Some(i) => i,
        None => gen_league(conn).id,
    };
    let team_id = match team_id {
        Some(i) => i,
        None => gen_team(conn, None).id,
    };

    let new_game = NewGame {
        team_id: team_id,
        league_id: league_id,
        venue: String::from(fake!(Lorem.word)),
        ident: Some(format!("{}", Uuid::new_v4())),
        matchday: None,
        result: None,
    };

    let game_id: Vec<i32> = diesel::insert_into(games)
        .values(&new_game)
        .returning(gid)
        .get_results(&**conn)
        .unwrap();

    games
        .find(game_id[0])
        .first(&**conn)
        .expect("Failed to fetch game!")
}

pub fn gen_player_game(
    conn: &db::Connection,
    game_id: Option<i32>,
    player_id: Option<i32>,
) -> PlayerGame {
    let game_id = match game_id {
        Some(i) => i,
        None => gen_game(conn, None, None).id,
    };
    let player_id = match player_id {
        Some(i) => i,
        None => gen_player(conn, None).id,
    };

    let player_game = NewPlayerGame { game_id, player_id };

    let player_game_id: Vec<i32> = diesel::insert_into(player_games)
        .values(&player_game)
        .returning(pgid)
        .get_results(&**conn)
        .unwrap();

    player_games
        .find(player_game_id[0])
        .first(&**conn)
        .expect("Failed to fetch player game!")
}
