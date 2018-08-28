//! Module representing a PlayerGame entity in the api database

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use schema::player_games;
use schema::player_games::dsl::*;

use models::game::Game;
use models::player::Player;

/// Struct representing a single row in the `player_games` table of the database
#[table_name = "player_games"]
#[belongs_to(Player)]
#[belongs_to(Game)]
#[derive(Serialize, Deserialize, Associations, Queryable, AsChangeset, Debug)]
pub struct PlayerGame {
    pub id: i32,
    pub game_id: i32,
    pub player_id: i32,
}

/// Struct used in `create` and `update` functions of the entity
#[table_name = "player_games"]
#[belongs_to(Player)]
#[belongs_to(Game)]
#[derive(Serialize, Deserialize, Insertable, Associations, Debug)]
pub struct NewPlayerGame {
    pub game_id: i32,
    pub player_id: i32,
}

impl PlayerGame {
    pub fn create(
        conn: &PgConnection,
        player_game: NewPlayerGame,
    ) -> Result<PlayerGame, diesel::result::Error> {
        diesel::insert_into(player_games)
            .values(player_game)
            .execute(conn)?;

        Ok(player_games.order(id.desc()).first(conn)?)
    }
}
