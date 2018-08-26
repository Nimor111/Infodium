//! Routes for interacting the `Team` entity

use db;
use models::team::{NewTeam, Team};

use rocket_contrib::Json;

use rocket::http::Status;

use guards::jwt::JwtGuard;
use responses::api_response::ApiResponse;
use responses::auth_response::AuthResponse;

/// GET - fetch all teams currently in the database
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::InternalServerError on database error
#[get("/")]
pub fn get_teams(conn: db::Connection) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(Some(json!(Team::all(&conn)?)), Status::Ok))
}

/// GET - fetch players in a team with an id of `id`
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::NotFound on non-existent resource
/// * Status::InternalServerError on database error
#[get("/<id>/players")]
pub fn get_team_players(conn: db::Connection, id: i32) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(
        Some(json!(&Team::get_team_players(id, &conn)?)),
        Status::Ok,
    ))
}

/// POST - create a new team with `team` data
/// # Returns
/// * HTTP 201 Created
///
/// # Errors
/// * Status::InternalServerError on database error
#[post("/", data = "<team>")]
pub fn create_team(
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    team: Json<NewTeam>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&Team::create(&conn, team.into_inner())?),
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
#[put("/<id>", data = "<team>")]
pub fn update_team(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    team: Json<NewTeam>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&Team::update(id, &conn, team.into_inner())?),
        Status::Ok,
    ))
}

/// DELETE - deletes a team in the database with an id of `id`
/// # Returns
/// * HTTP 200 Ok
///
/// # Errors
/// * Status::InternalServerError on database error
#[delete("/<id>")]
pub fn delete_team(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&Team::delete(id, &conn)?),
        Status::Ok,
    ))
}
