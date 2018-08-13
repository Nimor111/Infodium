use db;
use models::user::{NewUser, User};
use rocket::response::status;
use rocket_contrib::Json;

#[post("/", data = "<user>")]
pub fn register(
    conn: db::Connection,
    user: Result<NewUser, String>,
) -> Result<Json<String>, status::BadRequest<Json<String>>> {
    match user {
        Ok(u) => Ok(Json(User::create(&conn, u))),
        Err(e) => Err(status::BadRequest(Some(Json(e)))),
    }
}
