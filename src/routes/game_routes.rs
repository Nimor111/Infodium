use db;

use rocket_contrib::{Json, Value};

use models::game::{Game, NewGame};

use guards::jwt::JwtGuard;
use responses::auth_response::AuthResponse;

#[get("/")]
pub fn get_games(conn: db::Connection) -> Json<Value> {
    Json(json!(Game::all(&conn)))
}

#[post("/", data = "<game>")]
pub fn create_game(
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    game: Json<NewGame>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&Game::create(&conn, game.into_inner())),
    ))
}

#[put("/<id>", data = "<game>")]
pub fn update_game(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    game: Json<NewGame>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&Game::update(id, &conn, game.into_inner())),
    ))
}

#[delete("/<id>")]
pub fn delete_game(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!({ "success": Game::delete(id, &conn) }),
    ))
}
