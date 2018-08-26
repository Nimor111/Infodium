use db;

use rocket_contrib::Json;

use rocket::http::Status;

use models::game::{Game, NewGame};

use guards::jwt::JwtGuard;

use responses::api_response::ApiResponse;
use responses::auth_response::AuthResponse;

#[get("/")]
pub fn get_games(conn: db::Connection) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(
        Some(json!(&Game::all(&conn)?)),
        Status::Ok,
    ))
}

#[get("/<id>/players")]
pub fn get_game_players(id: i32, conn: db::Connection) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(
        Some(json!(&Game::get_game_players(id, &conn)?)),
        Status::Ok,
    ))
}

#[post("/", data = "<game>")]
pub fn create_game(
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    game: Json<NewGame>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&Game::create(&conn, game.into_inner())?),
        Status::Created,
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
        json!(&Game::update(id, &conn, game.into_inner())?),
        Status::Ok,
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
        json!(&Game::delete(id, &conn)?),
        Status::Ok,
    ))
}
