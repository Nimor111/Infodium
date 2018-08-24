use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use chrono::NaiveDate;

use schema::leagues;
use schema::leagues::dsl::*;

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
        league_id: i32,
        conn: &PgConnection,
        league: NewLeague,
    ) -> Result<League, diesel::result::Error> {
        diesel::update(leagues::table.find(league_id))
            .set(&League {
                id: league_id,
                name: league.name,
                country: league.country,
                current_matchday: league.current_matchday,
            }).execute(conn)?;

        Ok(leagues.find(league_id).first(conn)?)
    }

    pub fn delete(league_id: i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::delete(leagues::table.find(league_id)).execute(conn)?;

        Ok(())
    }
}
