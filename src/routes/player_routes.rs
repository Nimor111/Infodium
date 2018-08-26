//! Routes for interacting the `Player` entity

use db;
use models::player::{NewPlayer, Player};

use rocket_contrib::Json;

use rocket::http::Status;

use guards::jwt::JwtGuard;
use responses::api_response::ApiResponse;
use responses::auth_response::AuthResponse;

/// GET - fetch all players currently in the database
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::InternalServerError on database error
#[get("/")]
pub fn get_players(conn: db::Connection) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(
        Some(json!(Player::all(&conn)?)),
        Status::Ok,
    ))
}

/// POST - create a new player with `player` data
/// # Returns
/// * HTTP 201 Created
///
/// # Errors
/// * Status::InternalServerError on database error
#[post("/", data = "<player>")]
pub fn create_player(
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    player: Json<NewPlayer>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&Player::create(&conn, player.into_inner())?),
        Status::Created,
    ))
}

/// PUT - updates a player in the database with the `player` data and an id of `id`
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::NotFound on non-existent resource
/// * Status::InternalServerError on database error
#[put("/<id>", data = "<player>")]
pub fn update_player(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    player: Json<Player>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&Player::update(id, &conn, player.into_inner())?),
        Status::Ok,
    ))
}

/// DELETE - deletes a player in the database with an id of `id`
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::InternalServerError on database error
#[delete("/<id>")]
pub fn delete_player(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&Player::delete(id, &conn)?),
        Status::Ok,
    ))
}
