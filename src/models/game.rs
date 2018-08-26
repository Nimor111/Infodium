use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use uuid::Uuid;

use chrono::NaiveDate;

use schema::games;
use schema::games::dsl::*;
use schema::player_games;
use schema::players;

use models::league::League;
use models::player::Player;
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
    pub fn all(conn: &PgConnection) -> Result<Vec<Game>, diesel::result::Error> {
        let all_games = games.load::<Game>(conn)?;

        Ok(all_games)
    }

    pub fn create(conn: &PgConnection, game: NewGame) -> Result<Game, diesel::result::Error> {
        let new_game = NewGame {
            result: game.result,
            team_id: game.team_id,
            league_id: game.league_id,
            venue: game.venue,
            ident: Some(format!("{}", Uuid::new_v4())),
            matchday: game.matchday,
        };

        diesel::insert_into(games).values(&new_game).execute(conn)?;

        Ok(games.order(id.desc()).first(conn)?)
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
        diesel::delete(games.find(gid)).execute(conn)?;

        Ok(())
    }

    pub fn get_game_players(
        gid: i32,
        conn: &PgConnection,
    ) -> Result<Vec<Player>, diesel::result::Error> {
        let game_player_ids: Vec<i32> = player_games::table
            .filter(player_games::dsl::game_id.eq(gid))
            .select(player_games::dsl::player_id)
            .load(conn)?;

        let game_players = players::table
            .filter(players::dsl::id.eq_any(game_player_ids))
            .load(conn)?;

        Ok(game_players)
    }
}
