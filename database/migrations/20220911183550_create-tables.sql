CREATE TABLE Models (
    model_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    creation_date VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    author VARCHAR NOT NULL,
    image_path VARCHAR NOT NULL,
    scad_path VARCHAR NOT NULL
);

CREATE TABLE Parts (
    part_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    model_id INTEGER NOT NULL,
    FOREIGN KEY (model_id)
       REFERENCES Models (model_id)
);

CREATE TABLE Instances (
    path VARCHAR NOT NULL PRIMARY KEY,
    usage INTEGER NOT NULL DEFAULT 0,
    age INTEGER AUTO_INCREMENT,
    part_id INTEGER NOT NULL,
    FOREIGN KEY (part_id)
       REFERENCES Parts (part_id)
);

CREATE TABLE IntRangeParameters (
    parameter_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    default_value INTEGER NOT NULL,
    lower INTEGER NOT NULL,
    upper INTEGER NOT NULL,
    part_id INTEGER NOT NULL,
    FOREIGN KEY (part_id)
        REFERENCES Parts (part_id)
);

CREATE TABLE FloatRangeParameters (
    parameter_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    default_value REAL NOT NULL,
    lower REAL NOT NULL,
    upper REAL NOT NULL,
    part_id INTEGER NOT NULL,
    FOREIGN KEY (part_id)
        REFERENCES Parts (part_id)
);

CREATE TABLE StringLengthParameters (
    parameter_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    default_value VARCHAR NOT NULL,
    length INTEGER NOT NULL,
    part_id INTEGER NOT NULL,
    FOREIGN KEY (part_id)
        REFERENCES Parts (part_id)
);

CREATE TABLE BoolParameters (
    parameter_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    default_value BOOLEAN NOT NULL,
    part_id INTEGER NOT NULL,
    FOREIGN KEY (part_id)
        REFERENCES Parts (part_id)
);

CREATE TABLE IntListParameters (
    parameter_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    default_value INTEGER NOT NULL,
    part_id INTEGER NOT NULL,
    FOREIGN KEY (part_id)
        REFERENCES Parts (part_id)
);

CREATE TABLE IntListItems (
    item_id INTEGER PRIMARY KEY AUTOINCREMENT,
    value INTEGER NOT NULL,
    parameter_id INTEGER NOT NULL,
    FOREIGN KEY (parameter_id)
        REFERENCES IntListParameters (parameter_id)
);

CREATE TABLE FloatListParameters (
    parameter_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    default_value REAL NOT NULL,
    part_id INTEGER NOT NULL,
    FOREIGN KEY (part_id)
       REFERENCES Parts (part_id)
);

CREATE TABLE FloatListItems (
    item_id INTEGER PRIMARY KEY AUTOINCREMENT,
    value REAL NOT NULL,
    parameter_id INTEGER NOT NULL,
    FOREIGN KEY (parameter_id)
        REFERENCES FloatListParameters (parameter_id)
);

CREATE TABLE StringListParameters (
    parameter_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    default_value VARCHAR NOT NULL,
    part_id INTEGER NOT NULL,
    FOREIGN KEY (part_id)
        REFERENCES Parts (part_id)
);

CREATE TABLE StringListItems (
    item_id INTEGER PRIMARY KEY AUTOINCREMENT,
    value VARCHAR NOT NULL,
    parameter_id INTEGER NOT NULL,
    FOREIGN KEY (parameter_id)
        REFERENCES FloatListParameters (parameter_id)
);
