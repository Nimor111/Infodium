table! {
    leagues (id) {
        id -> Int4,
        name -> Text,
        country -> Text,
        current_matchday -> Nullable<Date>,
    }
}

table! {
    players (id) {
        id -> Int4,
        team_id -> Nullable<Int4>,
        name -> Text,
        position -> Text,
        country -> Text,
        nationality -> Text,
    }
}

table! {
    teams (id) {
        id -> Int4,
        league_id -> Int4,
        name -> Text,
        tla -> Text,
        address -> Nullable<Text>,
        website -> Nullable<Text>,
        facebook -> Nullable<Text>,
    }
}

joinable!(players -> teams (team_id));
joinable!(teams -> leagues (league_id));

allow_tables_to_appear_in_same_query!(
    leagues,
    players,
    teams,
);
