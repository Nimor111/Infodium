use db;
use models::league::{League, NewLeague};
use rocket_contrib::{Json, Value};

#[get("/")]
pub fn get_leagues(conn: db::Connection) -> Json<Value> {
    Json(json!(League::all(&conn)))
}

#[post("/", data = "<league>")]
pub fn create_league(conn: db::Connection, league: Json<NewLeague>) -> Json<League> {
    Json(League::create(&conn, league.into_inner()))
}

#[put("/<id>", data = "<league>")]
pub fn update_league(id: i32, conn: db::Connection, league: Json<NewLeague>) -> Json<League> {
    Json(League::update(id, &conn, league.into_inner()))
}

#[delete("/<id>")]
pub fn delete_league(id: i32, conn: db::Connection) -> Json<Value> {
    Json(json!({ "success": League::delete(id, &conn) }))
}
