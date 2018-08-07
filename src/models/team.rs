use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use schema::teams;
use schema::teams::dsl::*;

#[table_name = "teams"]
#[derive(Serialize, Deserialize, Associations, Queryable, AsChangeset, Debug)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub tla: String,
    pub address: Option<String>,
    pub website: Option<String>,
    pub facebook: Option<String>,
    pub league: i32,
}
