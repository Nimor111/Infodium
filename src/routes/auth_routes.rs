//! Routes for registration and login in the api

use rocket::http::Status;
use rocket_contrib::Json;

use diesel;
use diesel::prelude::*;

use bcrypt::verify;

use db;

use models::user::{NewUser, User};

use schema::users::dsl::*;

use utils::util::generate_jwt_token;

use responses::api_response::ApiResponse;

/// Attempt to register a user in the database.
/// # Errors
/// * Status::UnprocessableEntity if the data is invalid ( fails validation )
#[post("/register", data = "<user>")]
pub fn register(
    conn: db::Connection,
    user: Result<NewUser, String>,
) -> Result<ApiResponse, ApiResponse> {
    match user {
        Ok(u) => Ok(ApiResponse::new(
            Some(json!(&User::create(&conn, u)?)),
            Status::Ok,
        )),
        Err(_) => Err(ApiResponse::new(None, Status::UnprocessableEntity)),
    }
}

/// Attempt to login an existing user in the database with the provided credentials.
/// # Errors
/// * Status::Unauthorized if the user doesn't exist
/// * Status::Unauthorized if the password is wrong
/// * Status::BadRequest if the jwt token generation fails
#[post("/login", data = "<user>")]
pub fn login(conn: db::Connection, user: Json<NewUser>) -> Result<ApiResponse, ApiResponse> {
    let queried_user: Result<User, diesel::result::Error> = users
        .filter(email.eq(user.email.clone()))
        .select((id, email, username, password))
        .first(&*conn);

    let found_user = match queried_user {
        Ok(u) => u,
        Err(_) => return Err(ApiResponse::new(None, Status::Unauthorized)),
    };

    let valid = verify(&user.password, &found_user.password).unwrap();
    if !valid {
        return Err(ApiResponse::new(None, Status::Unauthorized));
    }

    match generate_jwt_token(json!({ "id": found_user.id })) {
        Ok(token) => Ok(ApiResponse::new(Some(json!(token)), Status::Ok)),
        Err(_) => Err(ApiResponse::new(None, Status::BadRequest)),
    }
}
