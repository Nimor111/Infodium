extern crate diesel;
extern crate infodium;
#[macro_use]
extern crate fake;
extern crate chrono;
extern crate uuid;

use diesel::prelude::*;
use diesel::sql_query;

use uuid::Uuid;

use infodium::models::game::*;
use infodium::models::league::*;
use infodium::models::player::*;
use infodium::models::team::*;
use infodium::models::user::*;

use infodium::schema::games::dsl::*;
use infodium::schema::leagues::dsl::*;
use infodium::schema::players::dsl::*;
use infodium::schema::teams::dsl::*;
use infodium::schema::users::dsl::*;

use infodium::db::*;

fn gen_user() -> NewUser {
    NewUser {
        username: fake!(Internet.user_name),
        password: String::from(fake!(Lorem.word)),
        email: fake!(Internet.free_email),
    }
}

fn gen_game(tid: i32, lid: i32) -> NewGame {
    NewGame {
        team_id: tid,
        league_id: lid,
        ident: format!("{}", Uuid::new_v4()),
        result: Some(String::from(fake!(Lorem.word))),
        venue: String::from(fake!(Lorem.word)),
        matchday: None,
    }
}

fn gen_player() -> NewPlayer {
    NewPlayer {
        name: fake!(Name.name),
        country: fake!(Name.name),
        position: String::from(fake!(Lorem.word)),
        nationality: String::from(fake!(Lorem.word)),
        team_id: None,
    }
}

fn gen_league() -> NewLeague {
    NewLeague {
        name: fake!(Name.name),
        country: fake!(Name.name),
        current_matchday: None,
    }
}

fn gen_team(lid: i32) -> NewTeam {
    NewTeam {
        name: fake!(Name.name),
        tla: String::from(fake!(Lorem.word)),
        address: Some(String::from(fake!(Address.street_address))),
        facebook: Some(String::from(fake!(Lorem.word))),
        website: Some(String::from(fake!(Lorem.word))),
        league_id: lid,
    }
}

fn main() -> Result<(), diesel::result::Error> {
    let conn = connect().get().unwrap();

    // Reset table serial ids
    sql_query("ALTER SEQUENCE leagues_id_seq RESTART WITH 1").execute(&*conn)?;
    sql_query("ALTER SEQUENCE teams_id_seq RESTART WITH 1").execute(&*conn)?;
    sql_query("ALTER SEQUENCE players_id_seq RESTART WITH 1").execute(&*conn)?;
    sql_query("ALTER SEQUENCE games_id_seq RESTART WITH 1").execute(&*conn)?;
    sql_query("ALTER SEQUENCE users_id_seq RESTART WITH 1").execute(&*conn)?;

    // Clear the database before running seed
    diesel::delete(players)
        .execute(&*conn)
        .expect("Error deleting players.");
    diesel::delete(games)
        .execute(&*conn)
        .expect("Error deleting games.");
    diesel::delete(teams)
        .execute(&*conn)
        .expect("Error deleting teams.");
    diesel::delete(leagues)
        .execute(&*conn)
        .expect("Error deleting leagues.");
    diesel::delete(users)
        .execute(&*conn)
        .expect("Error deleting users.");

    // Insert new records into db
    let new_players: Vec<NewPlayer> = (0..5).map(|_| gen_player()).collect();
    let new_leagues: Vec<NewLeague> = (0..5).map(|_| gen_league()).collect();
    let new_teams: Vec<NewTeam> = (0..5).map(|_| gen_team(1)).collect();
    let new_games: Vec<NewGame> = (0..5).map(|_| gen_game(1, 1)).collect();
    let new_users: Vec<NewUser> = (0..5).map(|_| gen_user()).collect();

    diesel::insert_into(players)
        .values(&new_players)
        .execute(&*conn)
        .expect("Error inserting players!");
    diesel::insert_into(leagues)
        .values(&new_leagues)
        .execute(&*conn)
        .expect("Error inserting leagues!");
    diesel::insert_into(teams)
        .values(&new_teams)
        .execute(&*conn)
        .expect("Error inserting teams!");
    diesel::insert_into(games)
        .values(&new_games)
        .execute(&*conn)
        .expect("Error inserting games!");
    diesel::insert_into(users)
        .values(&new_users)
        .execute(&*conn)
        .expect("Error inserting users!");

    Ok(())
}
