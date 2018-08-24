use rocket_contrib::Json;

#[catch(401)]
fn unauthorized_handler() -> Json<&'static str> {
    Json("No authentication token present!")
}
