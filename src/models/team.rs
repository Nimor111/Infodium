//! Module representing a Team entity in the api database

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use schema::players;
use schema::teams;
use schema::teams::dsl::*;

use models::league::League;
use models::player::Player;

/// Struct representing a single row in the `teams` table of the database
#[table_name = "teams"]
#[belongs_to(League)]
#[derive(Serialize, Deserialize, Associations, Queryable, AsChangeset, Debug)]
pub struct Team {
    pub id: i32,
    pub league_id: Option<i32>,
    pub name: String,
    pub tla: String,
    pub address: Option<String>,
    pub website: Option<String>,
    pub facebook: Option<String>,
}

/// Struct used in `create` and `update` functions of the entity
#[table_name = "teams"]
#[belongs_to(League)]
#[derive(Insertable, Associations, Deserialize, Serialize)]
pub struct NewTeam {
    pub league_id: Option<i32>,
    pub name: String,
    pub tla: String,
    pub address: Option<String>,
    pub website: Option<String>,
    pub facebook: Option<String>,
}

impl Team {
    pub fn all(conn: &PgConnection) -> Result<Vec<Team>, diesel::result::Error> {
        let all_teams = teams.load::<Team>(conn)?;

        Ok(all_teams)
    }

    pub fn create(conn: &PgConnection, team: NewTeam) -> Result<Team, diesel::result::Error> {
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
            .execute(conn)?;

        Ok(teams.order(id.desc()).first(conn)?)
    }

    pub fn update(
        tid: i32,
        conn: &PgConnection,
        team: NewTeam,
    ) -> Result<Team, diesel::result::Error> {
        let _team = teams.find(tid).first::<Team>(conn)?;

        diesel::update(teams::table.find(tid))
            .set(&Team {
                id: tid,
                name: team.name,
                tla: team.tla,
                address: team.address,
                website: team.website,
                facebook: team.facebook,
                league_id: team.league_id,
            }).execute(conn)?;

        Ok(teams.find(tid).first(conn)?)
    }

    pub fn delete(tid: i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        let _team = teams.find(tid).first::<Team>(conn)?;

        diesel::delete(teams::table.find(tid)).execute(conn)?;

        Ok(())
    }

    pub fn get_team_players(
        tid: i32,
        conn: &PgConnection,
    ) -> Result<Vec<Player>, diesel::result::Error> {
        let _team = teams.find(tid).first::<Team>(conn)?;

        let team_players = teams
            .inner_join(players::dsl::players)
            .filter(players::dsl::team_id.eq(tid))
            .select((
                players::dsl::id,
                players::dsl::team_id,
                players::dsl::name,
                players::dsl::position,
                players::dsl::country,
                players::dsl::nationality,
            )).load(conn)?;

        Ok(team_players)
    }
}
