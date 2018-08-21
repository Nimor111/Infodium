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

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, AsChangeset, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
}

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
    pub fn create(conn: &PgConnection, user: NewUser) -> String {
        let hashed_pass = hash(&user.password, 6).expect("Failed to hash!");
        let new_user = NewUser {
            email: String::from(&*user.email),
            username: user.username,
            password: hashed_pass,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
            .expect("Error creating new user!");

        let user = users
            .filter(email.eq(new_user.email))
            .first::<User>(&*conn)
            .expect("Error getting user!");

        generate_jwt_token(json!({"id": user.id})).unwrap()
    }

    pub fn update(user_id: i32, conn: &PgConnection, user: NewUser) {
        diesel::update(users::table.find(user_id))
            .set(&User {
                id: user_id,
                email: user.email,
                username: user.username,
                password: user.password,
            }).execute(conn)
            .expect("Error updating user!");
    }

    pub fn delete(user_id: i32, conn: &PgConnection) -> bool {
        diesel::delete(users::table.find(user_id))
            .execute(conn)
            .is_ok()
    }
}
