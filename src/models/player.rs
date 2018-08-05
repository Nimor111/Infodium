use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use schema::players;
use schema::players::dsl::*;

#[table_name = "players"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub position: String,
    pub country: String,
    pub nationality: String,
}

impl Player {
    pub fn all(conn: &MysqlConnection) -> Vec<Player> {
        players
            .load::<Player>(conn)
            .expect("Error loading players!")
    }

    pub fn create(conn: &MysqlConnection, player: Player) -> Player {
        diesel::insert_into(players::table)
            .values(&player)
            .execute(conn)
            .expect("Error creating new player!");

        players::table
            .order(players::id.desc())
            .first(conn)
            .expect("Error loading players!")
    }

    pub fn update(player_id: i32, conn: &MysqlConnection, player: Player) -> Player {
        diesel::update(players::table.find(player_id))
            .set(&player)
            .execute(conn)
            .expect("Error updating player!");

        players::table
            .find(player_id)
            .first(conn)
            .expect("Error getting player")
    }

    pub fn delete(player_id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(players::table.find(player_id))
            .execute(conn)
            .is_ok()
    }
}
