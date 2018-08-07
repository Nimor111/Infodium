CREATE TABLE players (
    id SERIAL PRIMARY KEY,
    team_id INTEGER REFERENCES teams (id),
    name TEXT NOT NULL,
    position TEXT NOT NULL,
    country TEXT NOT NULL,
    nationality TEXT NOT NULL
);
