extern crate diesel;
extern crate infodium;
#[macro_use]
extern crate fake;

use diesel::prelude::*;
use diesel::sql_query;
use infodium::models::league::*;
use infodium::models::player::*;
use infodium::models::team::*;

use infodium::schema::leagues::dsl::*;
use infodium::schema::players::dsl::*;
use infodium::schema::teams::dsl::*;

use infodium::db::*;

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

    // Clear the database before running seed
    diesel::delete(players)
        .execute(&*conn)
        .expect("Error deleting players.");
    diesel::delete(teams)
        .execute(&*conn)
        .expect("Error deleting teams.");
    diesel::delete(leagues)
        .execute(&*conn)
        .expect("Error deleting leagues.");

    // Insert new records into db
    let new_players: Vec<NewPlayer> = (0..5).map(|_| gen_player()).collect();
    let new_leagues: Vec<NewLeague> = (0..5).map(|_| gen_league()).collect();
    let new_teams: Vec<NewTeam> = (0..5).map(|_| gen_team(1)).collect();

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

    Ok(())
}
