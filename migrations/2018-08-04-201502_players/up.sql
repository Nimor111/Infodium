CREATE TABLE players (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    `name` VARCHAR(60) NOT NULL,
    position VARCHAR(10) NOT NULL,
    country VARCHAR(60) NOT NULL,
    nationality VARCHAR(60) NOT NULL,
    team INTEGER NOT NULL,
    FOREIGN KEY (team) REFERENCES teams(id)
);
