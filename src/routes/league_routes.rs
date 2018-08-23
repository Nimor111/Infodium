use rocket_contrib::{Json, Value};

use db;
use models::league::{League, NewLeague};

use guards::jwt::JwtGuard;
use responses::auth_response::AuthResponse;

#[post("/", data = "<league>")]
pub fn create_league(
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    league: Json<NewLeague>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&League::create(&conn, league.into_inner())),
    ))
}

#[get("/")]
pub fn get_leagues(conn: db::Connection) -> Json<Value> {
    Json(json!(League::all(&conn)))
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
        json!(&League::update(id, &conn, league.into_inner())),
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
        json!({ "success": League::delete(id, &conn) }),
    ))
}
