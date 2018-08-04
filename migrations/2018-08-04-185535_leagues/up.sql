CREATE TABLE leagues (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    `name` VARCHAR(60) NOT NULL,
    country VARCHAR(60) NOT NULL,
    current_matchday DATE
);
