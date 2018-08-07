use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use schema::players;
use schema::players::dsl::*;

use models::team::Team;

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
    pub fn all(conn: &PgConnection) -> Vec<Player> {
        players
            .load::<Player>(conn)
            .expect("Error loading players!")
    }

    pub fn create(conn: &PgConnection, player: NewPlayer) -> Player {
        let new_player = NewPlayer {
            name: player.name,
            position: player.position,
            country: player.country,
            nationality: player.nationality,
            team_id: player.team_id,
        };

        diesel::insert_into(players::table)
            .values(&new_player)
            .execute(conn)
            .expect("Error creating new player!");

        players::table
            .order(players::id.desc())
            .first(conn)
            .expect("Error loading players!")
    }

    pub fn update(player_id: i32, conn: &PgConnection, player: Player) -> Player {
        diesel::update(players::table.find(player_id))
            .set(&player)
            .execute(conn)
            .expect("Error updating player!");

        players::table
            .find(player_id)
            .first(conn)
            .expect("Error getting player")
    }

    pub fn delete(player_id: i32, conn: &PgConnection) -> bool {
        diesel::delete(players::table.find(player_id))
            .execute(conn)
            .is_ok()
    }
}
