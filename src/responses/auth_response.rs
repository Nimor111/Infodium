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
}

impl AuthResponse {
    pub fn new(token: Result<JwtGuard, ()>, data: Value) -> Self {
        AuthResponse { token, data }
    }
}

impl<'r> Responder<'r> for AuthResponse {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        let token = self.token;
        let data = self.data;

        match token {
            Ok(_) => Response::build()
                .status(Status::Ok)
                .sized_body(Cursor::new(data.to_string()))
                .header(ContentType::JSON)
                .ok(),
            Err(_) => Err(Status::Unauthorized),
        }
    }
}
