use db;
use models::user::{NewUser, User};
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::Json;

use guards::auth::AuthGuard;

use utils::utils::generate_jwt_token;

#[post("/register", data = "<user>")]
pub fn register(
    conn: db::Connection,
    user: Result<NewUser, String>,
) -> Result<Json<String>, status::Custom<Json<String>>> {
    match user {
        Ok(u) => Ok(Json(User::create(&conn, u))),
        Err(e) => Err(status::Custom(Status::UnprocessableEntity, Json(e))),
    }
}

#[post("/login", data = "<user>")]
pub fn login(
    _conn: db::Connection,
    user: Result<AuthGuard, String>,
) -> Result<Json<String>, status::Custom<Json<String>>> {
    match user {
        Ok(user) => match generate_jwt_token(json!({ "username": user.0 })) {
            Ok(token) => Ok(Json(token)),
            Err(e) => Err(status::Custom(Status::BadRequest, Json(format!("{:?}", e)))),
        },
        Err(e) => Err(status::Custom(Status::Unauthorized, Json(e))),
    }
}
