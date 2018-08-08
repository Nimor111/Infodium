#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate infodium;

use infodium::db;
use infodium::rocket;
use infodium::routes::league_routes;
use infodium::routes::player_routes;
use infodium::routes::team_routes;

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount(
            "/players",
            routes![
                player_routes::get_players,
                player_routes::create_player,
                player_routes::update_player,
                player_routes::delete_player
            ],
        ).mount(
            "/teams",
            routes![
                team_routes::get_teams,
                team_routes::create_team,
                team_routes::update_team,
                team_routes::delete_team
            ],
        ).mount(
            "/leagues",
            routes![
                league_routes::get_leagues,
                league_routes::create_league,
                league_routes::update_league,
                league_routes::delete_league
            ],
        ).launch();
}