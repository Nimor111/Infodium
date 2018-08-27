//! Module representing a Player entity in the api database

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use schema::players;
use schema::players::dsl::*;

use models::team::Team;

/// Struct representing a single row in the `players` table of the database
#[table_name = "players"]
#[derive(Serialize, Deserialize, Associations, Queryable, AsChangeset, Debug)]
#[belongs_to(Team)]
pub struct Player {
    pub id: i32,
    pub team_id: Option<i32>,
    pub name: String,
    pub position: String,
    pub country: String,
    pub nationality: String,
}

/// Struct used in `create` and `update` functions of the entity
#[table_name = "players"]
#[derive(Insertable, Associations, Deserialize, Serialize)]
#[belongs_to(Team)]
pub struct NewPlayer {
    pub name: String,
    pub team_id: Option<i32>,
    pub position: String,
    pub country: String,
    pub nationality: String,
}

impl Player {
    pub fn all(conn: &PgConnection) -> Result<Vec<Player>, diesel::result::Error> {
        let all_players = players.load::<Player>(conn)?;

        Ok(all_players)
    }

    pub fn create(conn: &PgConnection, player: NewPlayer) -> Result<Player, diesel::result::Error> {
        let new_player = NewPlayer {
            name: player.name,
            position: player.position,
            country: player.country,
            nationality: player.nationality,
            team_id: player.team_id,
        };

        diesel::insert_into(players::table)
            .values(&new_player)
            .execute(conn)?;

        Ok(players.order(id.desc()).first(conn)?)
    }

    pub fn update(
        pid: i32,
        conn: &PgConnection,
        player: Player,
    ) -> Result<Player, diesel::result::Error> {
        let _player = players.find(pid).first::<Player>(conn)?;

        diesel::update(players::table.find(pid))
            .set(&player)
            .execute(conn)?;

        Ok(players.find(pid).first(conn)?)
    }

    pub fn delete(pid: i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        let _player = players.find(pid).first::<Player>(conn)?;

        diesel::delete(players::table.find(pid)).execute(conn)?;

        Ok(())
    }
}
