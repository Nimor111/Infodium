use rocket::http::Status;
use rocket::response::status;

use rocket_contrib::{Json, Value};

use db;
use models::league::{League, NewLeague};

use guards::jwt::JwtGuard;

#[post("/", data = "<league>")]
pub fn create_league(
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
    league: Json<NewLeague>,
) -> Result<Json<League>, status::Custom<()>> {
    match jwt {
        Ok(_) => Ok(Json(League::create(&conn, league.into_inner()))),
        Err(_) => Err(status::Custom(Status::Unauthorized, ())),
    }
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
) -> Result<Json<League>, status::Custom<()>> {
    match jwt {
        Ok(_) => Ok(Json(League::update(id, &conn, league.into_inner()))),
        Err(_) => Err(status::Custom(Status::Unauthorized, ())),
    }
}

#[delete("/<id>")]
pub fn delete_league(
    id: i32,
    conn: db::Connection,
    jwt: Result<JwtGuard, ()>,
) -> Result<Json<Value>, status::Custom<()>> {
    match jwt {
        Ok(_) => Ok(Json(json!({ "success": League::delete(id, &conn) }))),
        Err(_) => Err(status::Custom(Status::Unauthorized, ())),
    }
}
