//! Module representing a User entity in the api database

use rocket::data::{self, FromData};
use rocket::http::Status;
use rocket::Outcome::*;
use rocket::{Data, Request};

use serde_json::from_reader;

use bcrypt::hash;

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use schema::users;
use schema::users::dsl::*;

use utils::util::generate_jwt_token;

use validator::Validate;

/// Struct representing a single row in the `users` table of the database
#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, AsChangeset, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
}

/// Struct used in `create` and `update` functions of the entity
#[table_name = "users"]
#[derive(Serialize, Deserialize, Insertable, Validate, Clone, Debug)]
pub struct NewUser {
    #[validate(email(message = "Email %s is not valid"))]
    pub email: String,
    pub username: String,
    #[validate(length(min = "6", message = "Password too short!"))]
    pub password: String,
}

impl FromData for NewUser {
    type Error = String;

    fn from_data(_request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let reader = data.open();
        match from_reader(reader).map(|val: NewUser| val) {
            Ok(value) => match value.validate() {
                Ok(_) => Success(value),
                Err(e) => Failure((Status::UnprocessableEntity, format!("{}", e))),
            },
            Err(e) => Failure((Status::BadRequest, e.to_string())),
        }
    }
}

impl User {
    pub fn create(conn: &PgConnection, user: NewUser) -> Result<String, diesel::result::Error> {
        let hashed_pass = hash(&user.password, 6).expect("Failed to hash!");
        let new_user = NewUser {
            email: String::from(&*user.email),
            username: user.username,
            password: hashed_pass,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        let user = users
            .filter(email.eq(new_user.email))
            .first::<User>(&*conn)?;

        Ok(generate_jwt_token(json!({"id": user.id})).unwrap())
    }

    pub fn update(
        uid: i32,
        conn: &PgConnection,
        user: NewUser,
    ) -> Result<(), diesel::result::Error> {
        let _user = users.find(uid).first::<User>(conn)?;

        diesel::update(users.find(uid))
            .set(&User {
                id: uid,
                email: user.email,
                username: user.username,
                password: user.password,
            }).execute(conn)?;

        Ok(())
    }

    pub fn delete(uid: i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        let _user = users.find(uid).first::<User>(conn)?;

        diesel::delete(users.find(uid)).execute(conn)?;

        Ok(())
    }
}
