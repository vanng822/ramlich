CREATE ROLE ramlich WITH LOGIN;

SET search_path TO public;

CREATE DATABASE ramlich OWNER ramlich;

CREATE TABLE request_event(
    id varchar(36) NOT NULL,
    url varchar(255) NOT NULL,
    timestamp bigint NOT NULL,
    response_time INTEGER NOT NULL,
    created_at timestamp DEFAULT current_timestamp,
    updated_at timestamp DEFAULT current_timestamp,
    PRIMARY KEY(id)
);