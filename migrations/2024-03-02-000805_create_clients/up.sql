-- Your SQL goes here
CREATE TABLE IF NOT EXISTS clients
(
    id BIGSERIAL PRIMARY KEY,
    ip TEXT NOT NULL,
    client_id TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ownership
(
    id BIGSERIAL PRIMARY KEY,
    owner_id TEXT NOT NULL,
    client_id TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS history
(
    id BIGSERIAL PRIMARY KEY,
    client_id TEXT NOT NULL,
    request_type TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS pair_records
(
    id BIGSERIAL PRIMARY KEY,
    history_id BIGINT NOT NULL,
    pair_type SMALLINT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL
);