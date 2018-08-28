//! Routes for interacting the `League` entity

use rocket_contrib::Json;

use rocket::http::Status;

use db;
use models::league::{League, NewLeague};

use guards::jwt::JwtGuard;
use responses::api_response::ApiResponse;
use responses::auth_response::AuthResponse;

/// POST - create a new league with `league` data
/// # Returns
/// * HTTP 201 Created
///
/// # Errors
/// * Status::InternalServerError on database error
#[post("/", data = "<league>")]
pub fn create_league(
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    league: Json<NewLeague>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&League::create(&conn, league.into_inner())?),
        Status::Created,
    ))
}

/// GET - fetch all leagues currently in the database
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::InternalServerError on database error
#[get("/")]
pub fn get_leagues(conn: db::Connection) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(
        Some(json!(League::all(&conn)?)),
        Status::Ok,
    ))
}

/// GET - fetch teams in a league with an id of `id`
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::NotFound on non-existent resource
/// * Status::InternalServerError on database error
#[get("/<id>/teams")]
pub fn get_league_teams(conn: db::Connection, id: i32) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(
        Some(json!(&League::get_league_teams(id, &conn)?)),
        Status::Ok,
    ))
}

#[get("/<id>")]
pub fn get_league(conn: db::Connection, id: i32) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(
        Some(json!(&League::get(id, &conn)?)),
        Status::Ok,
    ))
}

/// PUT - updates a game in the database with the `game` data and an id of `id`
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::NotFound on non-existent resource
/// * Status::InternalServerError on database error
#[put("/<id>", data = "<league>")]
pub fn update_league(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    league: Json<NewLeague>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&League::update(id, &conn, league.into_inner())?),
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
pub fn delete_league(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&League::delete(id, &conn)?),
        Status::Ok,
    ))
}
