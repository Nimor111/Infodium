use rocket_contrib::Json;

#[catch(401)]
fn unauthorized_handler() -> Json<&'static str> {
    Json("Incorrect authentication credentials!")
}

#[catch(404)]
fn not_found_handler() -> Json<&'static str> {
    Json("Resource not found!")
}

#[catch(422)]
fn unprocessable_entity_handler() -> Json<&'static str> {
    Json("Invalid request data!")
}

#[catch(500)]
fn internal_server_error_handler() -> Json<&'static str> {
    Json("Internal server error!")
}
