use db;
use models::team::{NewTeam, Team};

use rocket_contrib::{Json, Value};

use guards::jwt::JwtGuard;
use responses::auth_response::AuthResponse;

#[get("/")]
pub fn get_teams(conn: db::Connection) -> Json<Value> {
    Json(json!(Team::all(&conn)))
}

#[post("/", data = "<team>")]
pub fn create_team(
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    team: Json<NewTeam>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&Team::create(&conn, team.into_inner())),
    ))
}

#[put("/<id>", data = "<team>")]
pub fn update_team(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    team: Json<NewTeam>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!(&Team::update(id, &conn, team.into_inner())),
    ))
}

#[delete("/<id>")]
pub fn delete_team(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
) -> Result<AuthResponse, AuthResponse> {
    Ok(AuthResponse::new(
        jwt,
        json!({ "success": Team::delete(id, &conn) }),
    ))
}
