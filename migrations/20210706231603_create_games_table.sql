-- Add migration script here
CREATE TABLE games(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    turns TEXT NOT NULL
);