CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    league_id INTEGER REFERENCES leagues (id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    tla TEXT NOT NULL,
    address TEXT,
    website TEXT,
    facebook TEXT
);
