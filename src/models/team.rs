use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use schema::teams;
use schema::teams::dsl::*;

use models::league::League;

#[table_name = "teams"]
#[belongs_to(League)]
#[derive(Serialize, Deserialize, Associations, Queryable, AsChangeset, Debug)]
pub struct Team {
    pub id: i32,
    pub league_id: i32,
    pub name: String,
    pub tla: String,
    pub address: Option<String>,
    pub website: Option<String>,
    pub facebook: Option<String>,
}

#[table_name = "teams"]
#[belongs_to(League)]
#[derive(Insertable, Associations, Deserialize, Serialize)]
pub struct NewTeam {
    pub name: String,
    pub tla: String,
    pub address: Option<String>,
    pub website: Option<String>,
    pub facebook: Option<String>,
    pub league_id: i32,
}

impl Team {
    pub fn all(conn: &PgConnection) -> Vec<Team> {
        teams.load::<Team>(conn).expect("Error loading teams!")
    }

    pub fn create(conn: &PgConnection, team: NewTeam) -> Team {
        let new_team = NewTeam {
            name: team.name,
            tla: team.tla,
            address: team.address,
            website: team.website,
            facebook: team.facebook,
            league_id: team.league_id,
        };

        diesel::insert_into(teams::table)
            .values(&new_team)
            .execute(conn)
            .expect("Error creating new team!");

        teams::table
            .order(teams::id.desc())
            .first(conn)
            .expect("Error loading teams!")
    }

    pub fn update(team_id: i32, conn: &PgConnection, team: NewTeam) -> Team {
        diesel::update(teams::table.find(team_id))
            .set(&Team {
                id: team_id,
                name: team.name,
                tla: team.tla,
                address: team.address,
                website: team.website,
                facebook: team.facebook,
                league_id: team.league_id,
            }).execute(conn)
            .expect("Error updating team!");

        teams::table
            .find(team_id)
            .first(conn)
            .expect("Error getting team!")
    }

    pub fn delete(team_id: i32, conn: &PgConnection) -> bool {
        diesel::delete(teams::table.find(team_id))
            .execute(conn)
            .is_ok()
    }
}
