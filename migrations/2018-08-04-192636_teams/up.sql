CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    league_id INTEGER NOT NULL REFERENCES leagues (id),
    name TEXT NOT NULL,
    tla TEXT NOT NULL,
    address TEXT,
    website TEXT,
    facebook TEXT
);
