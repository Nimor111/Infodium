use rocket::data::Data;
use rocket::data::{self, FromData};
use rocket::http::Status;
use rocket::request::Request;
use rocket::Outcome;

use diesel::prelude::*;

use bcrypt::verify;

use serde_json::from_reader;

use schema::users::dsl::*;

use models::user::{NewUser, User};

use db;

pub struct AuthGuard(pub String);

impl FromData for AuthGuard {
    type Error = String;

    fn from_data(_request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let reader = data.open();
        match from_reader(reader).map(|val: NewUser| val) {
            Ok(u) => {
                let conn = db::connect().get().unwrap();
                let user = users
                    .filter(email.eq(u.email.clone()))
                    .select((id, username, password, email))
                    .first(&*conn);
                if let Err(_) = user {
                    return Outcome::Failure((
                        Status::Unauthorized,
                        String::from("Wrong credentials!"),
                    ));
                }
                let user: User = user.unwrap();

                let valid = verify(&u.password, &user.password).unwrap();
                if !valid {
                    return Outcome::Failure((
                        Status::Unauthorized,
                        String::from("Wrong credentials!"),
                    ));
                }

                Outcome::Success(AuthGuard(user.username))
            }
            Err(e) => Outcome::Failure((Status::BadRequest, format!("{:?}", e))),
        }
    }
}
