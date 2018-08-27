//! Module representing a League entity in the api database

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use chrono::NaiveDate;

use schema::leagues;
use schema::leagues::dsl::*;
use schema::teams;

use models::team::Team;

/// Struct representing a single row in the `leagues` table of the database
#[table_name = "leagues"]
#[derive(Serialize, Deserialize, Queryable, AsChangeset, Debug)]
pub struct League {
    pub id: i32,
    pub name: String,
    pub country: String,
    pub current_matchday: Option<NaiveDate>,
}

/// Struct used in `create` and `update` functions of the entity
#[table_name = "leagues"]
#[derive(Insertable, Deserialize, Serialize)]
pub struct NewLeague {
    pub name: String,
    pub country: String,
    pub current_matchday: Option<NaiveDate>,
}

impl League {
    pub fn all(conn: &PgConnection) -> Result<Vec<League>, diesel::result::Error> {
        let all_leagues = leagues.load::<League>(conn)?;

        Ok(all_leagues)
    }

    pub fn create(conn: &PgConnection, league: NewLeague) -> Result<League, diesel::result::Error> {
        let new_league = NewLeague {
            name: league.name,
            country: league.country,
            current_matchday: league.current_matchday,
        };

        diesel::insert_into(leagues::table)
            .values(&new_league)
            .execute(conn)?;

        Ok(leagues.order(id.desc()).first(conn)?)
    }

    pub fn update(
        lid: i32,
        conn: &PgConnection,
        league: NewLeague,
    ) -> Result<League, diesel::result::Error> {
        let _league = leagues.find(lid).first::<League>(conn)?;

        diesel::update(leagues.find(lid))
            .set(&League {
                id: lid,
                name: league.name,
                country: league.country,
                current_matchday: league.current_matchday,
            }).execute(conn)?;

        Ok(leagues.find(lid).first(conn)?)
    }

    pub fn delete(lid: i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        let _league = leagues.find(lid).first::<League>(conn)?;

        diesel::delete(leagues.find(lid)).execute(conn)?;

        Ok(())
    }

    pub fn get_league_teams(
        lid: i32,
        conn: &PgConnection,
    ) -> Result<Vec<Team>, diesel::result::Error> {
        let _league = leagues.find(lid).first::<League>(conn)?;

        let league_teams = leagues
            .inner_join(teams::dsl::teams)
            .filter(teams::dsl::league_id.eq(lid))
            .select((
                teams::dsl::id,
                teams::dsl::league_id,
                teams::dsl::name,
                teams::dsl::tla,
                teams::dsl::address,
                teams::dsl::website,
                teams::dsl::facebook,
            )).load(conn)?;

        Ok(league_teams)
    }
}
