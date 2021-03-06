//! Routes for interacting the `PlayerGame` entity

use db;
use models::player_game::{NewPlayerGame, PlayerGame};

use rocket_contrib::Json;

use rocket::http::Status;

use guards::jwt::JwtGuard;

use responses::auth_response::AuthResponse;

/// POST - create a new player game with `player_game` data
/// # Returns
/// * HTTP 201 Created
///
/// # Errors
/// * Status::InternalServerError on database error
#[post("/", data = "<player_game>")]
pub fn create_player_game(
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    player_game: Json<NewPlayerGame>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&PlayerGame::create(&conn, player_game.into_inner())?),
        Status::Created,
    ))
}
