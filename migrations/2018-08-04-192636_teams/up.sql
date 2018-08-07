CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    tla TEXT NOT NULL,
    address TEXT,
    website TEXT,
    facebook TEXT,
    league INTEGER NOT NULL REFERENCES leagues (id)
);
