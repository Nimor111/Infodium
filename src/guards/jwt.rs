use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket_contrib::Value;
use utils::utils::decode_token;

pub struct JwtGuard(Value);

impl<'a, 'r> FromRequest<'a, 'r> for JwtGuard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<JwtGuard, ()> {
        let data: Vec<_> = request.headers().get("x-auth").collect();
        if data.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let jwt = data[0];
        match decode_token(jwt.to_string()) {
            Ok(payload) => Outcome::Success(JwtGuard(payload)),
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
