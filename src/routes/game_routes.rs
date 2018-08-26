//! Routes for interacting with the `Game` entity

use db;

use rocket_contrib::Json;

use rocket::http::Status;

use models::game::{Game, NewGame};

use guards::jwt::JwtGuard;

use responses::api_response::ApiResponse;
use responses::auth_response::AuthResponse;

/// GET - fetch all games currently in the database
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::InternalServerError on database error
#[get("/")]
pub fn get_games(conn: db::Connection) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(
        Some(json!(&Game::all(&conn)?)),
        Status::Ok,
    ))
}

/// GET - fetch players in a game with an id of `id`
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::NotFound on non-existent resource
/// * Status::InternalServerError on database error
#[get("/<id>/players")]
pub fn get_game_players(id: i32, conn: db::Connection) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(
        Some(json!(&Game::get_game_players(id, &conn)?)),
        Status::Ok,
    ))
}

/// POST - create a new game with `game` data
/// # Returns
/// * HTTP 201 Created
///
/// # Errors
/// * Status::InternalServerError on database error
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

/// PUT - updates a game in the database with the `game` data and an id of `id`
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::NotFound on non-existent resource
/// * Status::InternalServerError on database error
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

/// DELETE - deletes a game in the database with an id of `id`
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::InternalServerError on database error
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
