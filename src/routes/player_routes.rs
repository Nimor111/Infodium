use db;
use models::player::{NewPlayer, Player};

use rocket::http::Status;
use rocket::response::status;

use rocket_contrib::{Json, Value};

use guards::jwt::JwtGuard;

#[get("/")]
pub fn get_players(conn: db::Connection) -> Json<Value> {
    Json(json!(Player::all(&conn)))
}

#[post("/", data = "<player>")]
pub fn create_player(
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    player: Json<NewPlayer>,
) -> Result<Json<Player>, status::Custom<()>> {
    match jwt {
        Ok(_) => Ok(Json(Player::create(&conn, player.into_inner()))),
        Err(_) => Err(status::Custom(Status::Unauthorized, ())),
    }
}

#[put("/<id>", data = "<player>")]
pub fn update_player(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    player: Json<Player>,
) -> Result<Json<Player>, status::Custom<()>> {
    match jwt {
        Ok(_) => Ok(Json(Player::update(id, &conn, player.into_inner()))),
        Err(_) => Err(status::Custom(Status::Unauthorized, ())),
    }
}

#[delete("/<id>")]
pub fn delete_player(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
) -> Result<Json<Value>, status::Custom<()>> {
    match jwt {
        Ok(_) => Ok(Json(json!({ "success": Player::delete(id, &conn) }))),
        Err(_) => Err(status::Custom(Status::Unauthorized, ())),
    }
}
