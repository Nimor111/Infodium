use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use uuid::Uuid;

use chrono::NaiveDate;

use schema::games;
use schema::games::dsl::*;

use models::league::League;
use models::team::Team;

#[table_name = "games"]
#[belongs_to(Team)]
#[belongs_to(League)]
#[derive(Serialize, Deserialize, Associations, Queryable, AsChangeset, Debug)]
pub struct Game {
    pub id: i32,
    pub team_id: i32,
    pub league_id: i32,
    pub ident: Option<String>,
    pub result: Option<String>,
    pub venue: String,
    pub matchday: Option<NaiveDate>,
}

#[table_name = "games"]
#[belongs_to(Team)]
#[belongs_to(League)]
#[derive(Insertable, Associations, Deserialize, Serialize)]
pub struct NewGame {
    pub team_id: i32,
    pub league_id: i32,
    pub ident: Option<String>,
    pub result: Option<String>,
    pub venue: String,
    pub matchday: Option<NaiveDate>,
}

impl Game {
    pub fn all(conn: &PgConnection) -> Vec<Game> {
        games.load::<Game>(conn).expect("Error loading games!")
    }

    pub fn create(conn: &PgConnection, game: NewGame) -> Game {
        let new_game = NewGame {
            result: game.result,
            team_id: game.team_id,
            league_id: game.league_id,
            venue: game.venue,
            ident: Some(format!("{}", Uuid::new_v4())),
            matchday: game.matchday,
        };

        diesel::insert_into(games::table)
            .values(&new_game)
            .execute(conn)
            .expect("Error creating new game!");

        games::table
            .order(games::id.desc())
            .first(conn)
            .expect("Error loading games!")
    }

    pub fn update(
        gid: i32,
        conn: &PgConnection,
        game: NewGame,
    ) -> Result<Game, diesel::result::Error> {
        diesel::update(games::table.find(gid))
            .set(&Game {
                id: gid,
                result: game.result,
                venue: game.venue,
                league_id: game.league_id,
                ident: game.ident,
                team_id: game.team_id,
                matchday: game.matchday,
            }).execute(conn)?;

        Ok(games.find(gid).first(conn)?)
    }

    pub fn delete(gid: i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::delete(games::table.find(gid)).execute(conn)?;

        Ok(())
    }
}
