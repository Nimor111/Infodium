use db;
use models::game::{Game, NewGame};
use rocket_contrib::{Json, Value};

#[get("/")]
pub fn get_games(conn: db::Connection) -> Json<Value> {
    Json(json!(Game::all(&conn)))
}

#[post("/", data = "<game>")]
pub fn create_game(conn: db::Connection, game: Json<NewGame>) -> Json<Game> {
    Json(Game::create(&conn, game.into_inner()))
}

#[put("/<id>", data = "<game>")]
pub fn update_game(id: i32, conn: db::Connection, game: Json<NewGame>) -> Json<Game> {
    Json(Game::update(id, &conn, game.into_inner()))
}

#[delete("/<id>")]
pub fn delete_game(id: i32, conn: db::Connection) -> Json<Value> {
    Json(json!({ "success": Game::delete(id, &conn) }))
}
