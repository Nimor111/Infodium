#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

#[macro_use]
pub extern crate serde_derive;
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
#[macro_use]
extern crate log;
extern crate fern;

pub extern crate r2d2;
pub extern crate r2d2_diesel;
pub extern crate rocket;

pub mod db;
pub mod handlers;
pub mod logger;
pub mod schema;

pub mod guards;
pub mod models;
pub mod responses;
pub mod routes;
pub mod utils;

use rocket::http::Method;
use rocket::Rocket;

use rocket_cors::{AllowedHeaders, AllowedOrigins};

/// Setup rocket instance - mounts routes, sets up CORS options and registers custom error handlers.
/// * Returns rocket instance and a db connection pool
pub fn rocket() -> (Rocket, db::Pool) {
    let (allowed_origins, _) = AllowedOrigins::some(&["http://localhost:3000"]);

    let options = rocket_cors::Cors {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "x-auth",
            "content-type",
        ]),
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
                routes::team_routes::delete_team,
                routes::team_routes::get_team_players
            ],
        ).mount(
            "/leagues",
            routes![
                routes::league_routes::get_leagues,
                routes::league_routes::get_league,
                routes::league_routes::create_league,
                routes::league_routes::update_league,
                routes::league_routes::delete_league,
                routes::league_routes::get_league_teams
            ],
        ).mount(
            "/games",
            routes![
                routes::game_routes::get_games,
                routes::game_routes::create_game,
                routes::game_routes::update_game,
                routes::game_routes::delete_game,
                routes::game_routes::get_game_players
            ],
        ).mount(
            "/auth",
            routes![routes::auth_routes::register, routes::auth_routes::login],
        ).mount(
            "/player-games",
            routes![routes::player_game_routes::create_player_game],
        ).attach(options)
        .catch(catchers![
            handlers::unauthorized_handler,
            handlers::not_found_handler,
            handlers::unprocessable_entity_handler,
            handlers::internal_server_error_handler
        ]);

    (rocket, db_pool)
}
