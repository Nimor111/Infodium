extern crate infodium;
extern crate rocket;
#[macro_use]
extern crate fake;
#[macro_use]
extern crate rocket_contrib;
extern crate diesel;
extern crate serde_derive;
extern crate serde_json;

use diesel::prelude::*;

use rocket::http::{ContentType, Status};

use infodium::db;
use infodium::models::user::User;
use infodium::schema::users::dsl::*;

#[macro_use]
mod common;
mod seed;

use common::DB_LOCK;
use seed::gen_user;

fn get_all_users(conn: &db::Connection) -> Vec<User> {
    users.load::<User>(&**conn).expect("Error loading users!")
}

#[test]
fn test_registers_a_user_successfully() {
    run_test!(|client, conn, _jwt| {
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
    run_test!(|client, conn, _jwt| {
        let user = gen_user(&conn);

        let body = json!({
            "email": user.email,
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
    run_test!(|client, _conn, _jwt| {
        let body = json!({
            "email": fake!(Internet.free_email),
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
            Some("\"Incorrect authentication credentials!\"".to_string())
        );
    })
}

#[test]
fn test_login_is_not_successful_with_invalid_password() {
    run_test!(|client, conn, _jwt| {
        let user = gen_user(&conn);

        let body = json!({
            "email": user.email,
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
            Some("\"Incorrect authentication credentials!\"".to_string())
        );
    })
}
