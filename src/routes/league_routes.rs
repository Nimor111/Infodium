use rocket_contrib::Json;

use rocket::http::Status;

use db;
use models::league::{League, NewLeague};

use guards::jwt::JwtGuard;
use responses::api_response::ApiResponse;
use responses::auth_response::AuthResponse;

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

#[get("/")]
pub fn get_leagues(conn: db::Connection) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(
        Some(json!(League::all(&conn)?)),
        Status::Ok,
    ))
}

#[get("/<id>/teams")]
pub fn get_league_teams(conn: db::Connection, id: i32) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(
        Some(json!(&League::get_league_teams(id, &conn)?)),
        Status::Ok,
    ))
}

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

#[delete("/<id>")]
pub fn delete_league(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&League::delete(id, &conn)?),
        Status::NoContent,
    ))
}
