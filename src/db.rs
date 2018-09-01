use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

use diesel::pg::PgConnection;

/// Type alias for a r2d2 connection pool
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Attempt to connect to the database provided by the `DATABASE_URL` environment variable.
/// # Panics
/// * `DATABASE_URL` does not exist.
pub fn connect() -> Result<Pool, r2d2::PoolError> {
    let database_url = dotenv!("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager)?;

    Ok(pool)
}

/// Wrapper struct for a pooled database connection
pub struct Connection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
