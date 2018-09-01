table! {
    games (id) {
        id -> Int4,
        team_id -> Int4,
        league_id -> Int4,
        ident -> Nullable<Text>,
        result -> Nullable<Text>,
        venue -> Text,
        matchday -> Nullable<Date>,
    }
}

table! {
    leagues (id) {
        id -> Int4,
        name -> Text,
        country -> Text,
        current_matchday -> Nullable<Date>,
    }
}

table! {
    player_games (id) {
        id -> Int4,
        game_id -> Int4,
        player_id -> Int4,
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
        league_id -> Nullable<Int4>,
        name -> Text,
        tla -> Text,
        address -> Nullable<Text>,
        website -> Nullable<Text>,
        facebook -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Nullable<Text>,
        password -> Text,
    }
}

joinable!(games -> leagues (league_id));
joinable!(games -> teams (team_id));
joinable!(player_games -> games (game_id));
joinable!(player_games -> players (player_id));
joinable!(players -> teams (team_id));
joinable!(teams -> leagues (league_id));

allow_tables_to_appear_in_same_query!(games, leagues, player_games, players, teams, users,);
