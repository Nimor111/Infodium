CREATE TABLE games (
    id SERIAL PRIMARY KEY,
    team_id INTEGER REFERENCES teams (id) NOT NULL,
    league_id INTEGER REFERENCES leagues (id) NOT NULL,
    ident TEXT NOT NULL,
    result TEXT,
    venue TEXT NOT NULL,
    matchday DATE
);
