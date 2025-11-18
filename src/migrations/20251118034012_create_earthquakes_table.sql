-- Add migration script here
CREATE TABLE IF NOT EXISTS earthquakes (
    id TEXT PRIMARY KEY,
    magnitude REAL NOT NULL,
    place TEXT NOT NULL,
    time BIGINT NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    depth DOUBLE PRECISION NOT NULL
);
