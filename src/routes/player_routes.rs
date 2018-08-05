use db;
use models::player::{NewPlayer, Player};
use rocket_contrib::{Json, Value};

#[get("/")]
pub fn get_players(conn: db::Connection) -> Json<Value> {
    Json(json!(Player::all(&conn)))
}

#[post("/", data = "<player>")]
pub fn create_player(conn: db::Connection, player: Json<NewPlayer>) -> Json<Player> {
    Json(Player::create(&conn, player.into_inner()))
}

#[put("/<id>", data = "<player>")]
pub fn update_player(id: i32, conn: db::Connection, player: Json<Player>) -> Json<Player> {
    Json(Player::update(id, &conn, player.into_inner()))
}

#[delete("/<id>")]
pub fn delete_player(id: i32, conn: db::Connection) -> Json<Value> {
    Json(json!({ "success": Player::delete(id, &conn) }))
}
