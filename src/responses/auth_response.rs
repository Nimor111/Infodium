use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket::Request;

use rocket_contrib::Value;

use guards::jwt::JwtGuard;

#[derive(Debug)]
pub struct AuthResponse {
    token: Result<JwtGuard, ()>,
    data: Value,
    status: Status,
}

impl AuthResponse {
    pub fn new(token: Result<JwtGuard, ()>, data: Value, status: Status) -> Self {
        AuthResponse {
            token,
            data,
            status,
        }
    }
}

impl<'r> Responder<'r> for AuthResponse {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        let token = self.token;
        let data = self.data;
        let status = self.status;

        match token {
            Ok(_) => Response::build()
                .status(status)
                .sized_body(Cursor::new(data.to_string()))
                .header(ContentType::JSON)
                .ok(),
            Err(_) => Err(Status::Unauthorized),
        }
    }
}
