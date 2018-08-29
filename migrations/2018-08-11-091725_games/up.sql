CREATE TABLE games (
    id SERIAL PRIMARY KEY,
    team_id INTEGER REFERENCES teams (id) ON DELETE CASCADE NOT NULL,
    league_id INTEGER REFERENCES leagues (id) ON DELETE CASCADE NOT NULL,
    ident TEXT,
    result TEXT,
    venue TEXT NOT NULL,
    matchday DATE
);
