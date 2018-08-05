#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate infodium;

use infodium::db;
use infodium::rocket;
use infodium::routes::player_routes;

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
        ).launch();
}
