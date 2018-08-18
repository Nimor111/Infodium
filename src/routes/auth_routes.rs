use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::Json;

use diesel::prelude::*;

use bcrypt::verify;

use db;

use models::user::{NewUser, User};

use schema::users::dsl::*;

use utils::util::generate_jwt_token;

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
    conn: db::Connection,
    user: Json<NewUser>,
) -> Result<Json<String>, status::Custom<Json<&'static str>>> {
    let queried_user: Result<User, status::Custom<Json<&str>>> = users
        .filter(email.eq(user.email.clone()))
        .select((id, username, password, email))
        .first(&*conn)
        .map_err(|_| status::Custom(Status::Unauthorized, Json("Wrong credentials!")));

    let found_user = match queried_user {
        Ok(u) => u,
        Err(e) => return Err(e),
    };

    let valid = verify(&user.password, &found_user.password).unwrap();
    if !valid {
        return Err(status::Custom(
            Status::Unauthorized,
            Json("Wrong credentials!"),
        ));
    }

    match generate_jwt_token(json!({ "id": found_user.id })) {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(status::Custom(
            Status::BadRequest,
            Json("Something went wrong!"),
        )),
    }
}
