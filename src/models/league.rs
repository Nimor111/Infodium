use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use chrono::NaiveDate;

use schema::leagues;
use schema::leagues::dsl::*;
use schema::teams;

use models::team::Team;

#[table_name = "leagues"]
#[derive(Serialize, Deserialize, Queryable, AsChangeset, Debug)]
pub struct League {
    pub id: i32,
    pub name: String,
    pub country: String,
    pub current_matchday: Option<NaiveDate>,
}

#[table_name = "leagues"]
#[derive(Insertable, Deserialize, Serialize)]
pub struct NewLeague {
    pub name: String,
    pub country: String,
    pub current_matchday: Option<NaiveDate>,
}

impl League {
    pub fn all(conn: &PgConnection) -> Vec<League> {
        leagues
            .load::<League>(conn)
            .expect("Error loading leagues!")
    }

    pub fn create(conn: &PgConnection, league: NewLeague) -> League {
        let new_league = NewLeague {
            name: league.name,
            country: league.country,
            current_matchday: league.current_matchday,
        };

        diesel::insert_into(leagues::table)
            .values(&new_league)
            .execute(conn)
            .expect("Error creating new league!");

        leagues::table
            .order(leagues::id.desc())
            .first(conn)
            .expect("Error loading leagues!")
    }

    pub fn update(
        lid: i32,
        conn: &PgConnection,
        league: NewLeague,
    ) -> Result<League, diesel::result::Error> {
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
        diesel::delete(leagues.find(lid)).execute(conn)?;

        Ok(())
    }

    pub fn get_league_teams(
        lid: i32,
        conn: &PgConnection,
    ) -> Result<Vec<Team>, diesel::result::Error> {
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
