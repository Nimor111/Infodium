extern crate infodium;
extern crate rocket;
#[macro_use]
extern crate fake;
#[macro_use]
extern crate rocket_contrib;
extern crate bcrypt;
extern crate diesel;
extern crate serde_derive;
extern crate serde_json;

use self::bcrypt::hash;

use self::diesel::prelude::*;

use self::rocket::http::{ContentType, Status};

use self::infodium::db;
use self::infodium::models::user::{NewUser, User};
use self::infodium::schema::users::dsl::*;

#[macro_use]
mod common;

use common::DB_LOCK;

fn get_all_users(conn: &db::Connection) -> Vec<User> {
    users.load::<User>(&**conn).expect("Error loading users!")
}

fn gen_user(conn: &db::Connection) -> User {
    let hashed_pass = hash("password123", 6).expect("Failed to hash!");

    let new_user = NewUser {
        email: fake!(Internet.free_email),
        username: fake!(Internet.user_name),
        password: hashed_pass,
    };

    let user_id: Vec<i32> = diesel::insert_into(users)
        .values(&new_user)
        .returning(id)
        .get_results(&**conn)
        .unwrap();

    users
        .find(user_id[0])
        .first(&**conn)
        .expect("Failed to fetch user!")
}

#[test]
fn test_registers_a_user_successfully() {
    run_test!(|client, conn| {
        let user_count = get_all_users(&conn).len();

        let body = json!({
            "username": fake!(Internet.user_name),
            "email": fake!(Internet.free_email),
            "password": "password123",
        }).to_string();

        let mut response = client
            .post("/auth/register")
            .header(ContentType::JSON)
            .body(body)
            .dispatch();

        let new_user_count = get_all_users(&conn).len();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(new_user_count, user_count + 1);
        assert!(response.body().is_some());
    })
}

#[test]
fn test_login_is_successful_with_valid_credentials() {
    run_test!(|client, conn| {
        let user = gen_user(&conn);

        let body = json!({
            "email": user.email,
            "username": user.username,
            "password": "password123",
        }).to_string();

        let mut response = client
            .post("/auth/login")
            .header(ContentType::JSON)
            .body(body)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
    })
}

#[test]
fn test_login_is_not_successful_with_invalid_email() {
    run_test!(|client, conn| {
        let user = gen_user(&conn);

        let body = json!({
            "email": fake!(Internet.free_email),
            "username": user.username,
            "password": "password123"
        }).to_string();

        let mut response = client
            .post("/auth/login")
            .header(ContentType::JSON)
            .body(body)
            .dispatch();

        assert_eq!(response.status(), Status::Unauthorized);
        assert_eq!(
            response.body_string(),
            Some("\"Wrong credentials!\"".to_string())
        );
    })
}

#[test]
fn test_login_is_not_successful_with_invalid_password() {
    run_test!(|client, conn| {
        let user = gen_user(&conn);

        let body = json!({
            "email": user.email,
            "username": user.username,
            "password": "password234"
        }).to_string();

        let mut response = client
            .post("/auth/login")
            .header(ContentType::JSON)
            .body(body)
            .dispatch();

        assert_eq!(response.status(), Status::Unauthorized);
        assert_eq!(
            response.body_string(),
            Some("\"Wrong credentials!\"".to_string())
        );
    })
}
