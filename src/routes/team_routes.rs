use db;
use models::team::{NewTeam, Team};
use rocket_contrib::{Json, Value};

#[get("/")]
pub fn get_teams(conn: db::Connection) -> Json<Value> {
    Json(json!(Team::all(&conn)))
}

#[post("/", data = "<team>")]
pub fn create_team(conn: db::Connection, team: Json<NewTeam>) -> Json<Team> {
    Json(Team::create(&conn, team.into_inner()))
}

#[put("/<id>", data = "<team>")]
pub fn update_team(id: i32, conn: db::Connection, team: Json<NewTeam>) -> Json<Team> {
    Json(Team::update(id, &conn, team.into_inner()))
}

#[delete("/<id>")]
pub fn delete_team(id: i32, conn: db::Connection) -> Json<Value> {
    Json(json!({ "success": Team::delete(id, &conn) }))
}
