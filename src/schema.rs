table! {
    leagues (id) {
        id -> Integer,
        name -> Varchar,
        country -> Varchar,
        current_matchday -> Nullable<Date>,
    }
}

table! {
    players (id) {
        id -> Integer,
        name -> Varchar,
        position -> Varchar,
        country -> Varchar,
        nationality -> Varchar,
    }
}

table! {
    teams (id) {
        id -> Integer,
        name -> Varchar,
        tla -> Varchar,
        address -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        facebook -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(leagues, players, teams,);
