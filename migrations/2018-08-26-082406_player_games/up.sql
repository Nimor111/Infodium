CREATE TABLE player_games (
    id SERIAL PRIMARY KEY,
    game_id INTEGER REFERENCES games (id) NOT NULL,
    player_id INTEGER REFERENCES players (id) NOT NULL
);
