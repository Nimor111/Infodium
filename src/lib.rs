#![feature(attr_literals)]
#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

#[macro_use]
pub extern crate serde_derive;
#[macro_use]
pub extern crate rocket_contrib;
#[macro_use]
pub extern crate diesel;
#[macro_use]
pub extern crate dotenv_codegen;
#[macro_use]
pub extern crate serde_json;
pub extern crate frank_jwt;

extern crate bcrypt;
extern crate validator;
#[macro_use]
extern crate validator_derive;
extern crate chrono;
extern crate rocket_cors;
extern crate uuid;

pub extern crate r2d2;
pub extern crate r2d2_diesel;
pub extern crate rocket;

pub mod db;
pub mod handlers;
pub mod schema;

pub mod guards;
pub mod models;
pub mod responses;
pub mod routes;
pub mod utils;

use rocket::http::Method;
use rocket::Rocket;

use rocket_cors::{AllowedHeaders, AllowedOrigins};

pub fn rocket() -> (Rocket, db::Pool) {
    let (allowed_origins, _) = AllowedOrigins::some(&["http://localhost:3000"]);

    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    let db_pool = db::connect().unwrap();
    let rocket = rocket::ignite()
        .manage(db_pool.clone())
        .mount(
            "/players",
            routes![
                routes::player_routes::get_players,
                routes::player_routes::create_player,
                routes::player_routes::update_player,
                routes::player_routes::delete_player
            ],
        ).mount(
            "/teams",
            routes![
                routes::team_routes::get_teams,
                routes::team_routes::create_team,
                routes::team_routes::update_team,
                routes::team_routes::delete_team
            ],
        ).mount(
            "/leagues",
            routes![
                routes::league_routes::get_leagues,
                routes::league_routes::create_league,
                routes::league_routes::update_league,
                routes::league_routes::delete_league
            ],
        ).mount(
            "/games",
            routes![
                routes::game_routes::get_games,
                routes::game_routes::create_game,
                routes::game_routes::update_game,
                routes::game_routes::delete_game
            ],
        ).mount(
            "/auth",
            routes![routes::auth_routes::register, routes::auth_routes::login],
        ).attach(options)
        .catch(catchers![handlers::unauthorized_handler]);

    (rocket, db_pool)
}
