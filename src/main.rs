#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate infodium;

use infodium::db;
use infodium::models::player::Player;
use infodium::rocket;
use infodium::rocket_contrib::*;
use infodium::rocket_contrib::{Json, Value};

#[get("/")]
fn index() -> Json<Value> {
    Json(json!({"greeting": "Hello world!"}))
}

#[get("/")]
fn get_players(conn: db::Connection) -> Json<Value> {
    Json(json!(Player::all(&conn)))
}

#[post("/", data = "<player>")]
fn create_player(conn: db::Connection, player: Json<Player>) -> Json<Player> {
    Json(Player::create(&conn, player.into_inner()))
}

#[put("/<id>", data = "<player>")]
fn update_player(id: i32, conn: db::Connection, player: Json<Player>) -> Json<Player> {
    Json(Player::update(id, &conn, player.into_inner()))
}

#[delete("/<id>")]
fn delete_player(id: i32, conn: db::Connection) -> Json<Value> {
    Json(json!({ "success": Player::delete(id, &conn) }))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![index])
        .mount(
            "/players",
            routes![get_players, create_player, update_player, delete_player],
        ).launch();
}
