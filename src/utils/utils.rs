use frank_jwt::{decode, encode, Algorithm, Error};
use rocket_contrib::Value;

static SECRET_KEY: &'static str = dotenv!("SECRET_KEY");

pub fn generate_jwt_token(payload: Value) -> Result<String, Error> {
    encode(
        json!({}),
        &SECRET_KEY.to_string(),
        &payload,
        Algorithm::HS256,
    )
}

pub fn decode_token(jwt: String) -> Result<Value, Error> {
    let (_, payload) = decode(&jwt, &SECRET_KEY.to_string(), Algorithm::HS256)?;
    Ok(payload)
}
