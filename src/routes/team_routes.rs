use db;
use models::team::{NewTeam, Team};

use rocket::http::Status;
use rocket::response::status;

use rocket_contrib::{Json, Value};

use guards::jwt::JwtGuard;

#[get("/")]
pub fn get_teams(conn: db::Connection) -> Json<Value> {
    Json(json!(Team::all(&conn)))
}

#[post("/", data = "<team>")]
pub fn create_team(
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    team: Json<NewTeam>,
) -> Result<Json<Team>, status::Custom<()>> {
    match jwt {
        Ok(_) => Ok(Json(Team::create(&conn, team.into_inner()))),
        Err(_) => Err(status::Custom(Status::Unauthorized, ())),
    }
}

#[put("/<id>", data = "<team>")]
pub fn update_team(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    team: Json<NewTeam>,
) -> Result<Json<Team>, status::Custom<()>> {
    match jwt {
        Ok(_) => Ok(Json(Team::update(id, &conn, team.into_inner()))),
        Err(_) => Err(status::Custom(Status::Unauthorized, ())),
    }
}

#[delete("/<id>")]
pub fn delete_team(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
) -> Result<Json<Value>, status::Custom<()>> {
    match jwt {
        Ok(_) => Ok(Json(json!({ "success": Team::delete(id, &conn) }))),
        Err(_) => Err(status::Custom(Status::Unauthorized, ())),
    }
}
