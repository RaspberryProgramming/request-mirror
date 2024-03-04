-- Your SQL goes here
CREATE TABLE IF NOT EXISTS clients
(
    id SERIAL PRIMARY KEY,
    ip TEXT NOT NULL,
    mirror_id TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ownership
(
    id SERIAL PRIMARY KEY,
    owner_id TEXT NOT NULL,
    client_id TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS history
(
    id SERIAL PRIMARY KEY,
    client_id TEXT NOT NULL,
    request_type TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS pair_records
(
    id SERIAL PRIMARY KEY,
    history_id INTEGER NOT NULL,
    pair_type INTEGER NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL
);