use db;

use rocket::http::Status;
use rocket::response::status;

use rocket_contrib::{Json, Value};

use models::game::{Game, NewGame};

use guards::jwt::JwtGuard;

#[get("/")]
pub fn get_games(conn: db::Connection) -> Json<Value> {
    Json(json!(Game::all(&conn)))
}

#[post("/", data = "<game>")]
pub fn create_game(
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    game: Json<NewGame>,
) -> Result<Json<Game>, status::Custom<()>> {
    match jwt {
        Ok(_) => Ok(Json(Game::create(&conn, game.into_inner()))),
        Err(_) => Err(status::Custom(Status::Unauthorized, ())),
    }
}

#[put("/<id>", data = "<game>")]
pub fn update_game(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    game: Json<NewGame>,
) -> Result<Json<Game>, status::Custom<()>> {
    match jwt {
        Ok(_) => Ok(Json(Game::update(id, &conn, game.into_inner()))),
        Err(_) => Err(status::Custom(Status::Unauthorized, ())),
    }
}

#[delete("/<id>")]
pub fn delete_game(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
) -> Result<Json<Value>, status::Custom<()>> {
    match jwt {
        Ok(_) => Ok(Json(json!({ "success": Game::delete(id, &conn) }))),
        Err(_) => Err(status::Custom(Status::Unauthorized, ())),
    }
}
