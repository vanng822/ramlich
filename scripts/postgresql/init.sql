CREATE ROLE ramlich WITH LOGIN;

SET search_path TO public;

CREATE DATABASE ramlich OWNER ramlich;

CREATE TABLE request_event(
    id UUID NOT NULL,
    url VARCHAR(255) NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    response_time INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT current_timestamp,
    updated_at TIMESTAMP DEFAULT current_timestamp,
    PRIMARY KEY(id)
);