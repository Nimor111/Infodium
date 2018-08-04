CREATE TABLE teams (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    `name` VARCHAR(60) NOT NULL,
    tla VARCHAR(10) NOT NULL,
    address VARCHAR(255),
    website VARCHAR(255),
    facebook VARCHAR(255),
    league INTEGER NOT NULL,
    FOREIGN KEY (league) REFERENCES leagues(id)
);
