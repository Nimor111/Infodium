use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket::Request;

use diesel::result::Error as DieselError;

use rocket_contrib::Value;

#[derive(Debug)]
pub struct ApiResponse {
    data: Option<Value>,
    status: Status,
}

impl ApiResponse {
    pub fn new(data: Option<Value>, status: Status) -> Self {
        ApiResponse { data, status }
    }
}

impl From<DieselError> for ApiResponse {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => ApiResponse::new(None, Status::NotFound),
            _ => ApiResponse::new(None, Status::InternalServerError),
        }
    }
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        let data = self.data;

        match self.status {
            Status::Ok => Response::build()
                .status(self.status)
                .sized_body(Cursor::new(data.unwrap().to_string()))
                .header(ContentType::JSON)
                .ok(),
            _ => Err(self.status),
        }
    }
}
