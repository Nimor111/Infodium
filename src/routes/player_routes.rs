use db;
use models::player::{NewPlayer, Player};

use rocket_contrib::Json;

use rocket::http::Status;

use guards::jwt::JwtGuard;
use responses::api_response::ApiResponse;
use responses::auth_response::AuthResponse;

#[get("/")]
pub fn get_players(conn: db::Connection) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new(
        Some(json!(Player::all(&conn)?)),
        Status::Ok,
    ))
}

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

#[delete("/<id>")]
pub fn delete_player(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&Player::delete(id, &conn)?),
        Status::NoContent,
    ))
}
