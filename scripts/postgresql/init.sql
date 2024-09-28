CREATE ROLE ramlich WITH LOGIN;

SET search_path TO public;

CREATE DATABASE ramlich OWNER ramlich;
CREATE DATABASE unleash OWNER ramlich;

CREATE TABLE request_event(
    id UUID NOT NULL,
    url VARCHAR(255) NOT NULL,
    requested_at TIMESTAMP WITH TIME ZONE NOT NULL,
    response_time BIGINT NOT NULL,
    status_code INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT current_timestamp,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT current_timestamp,
    PRIMARY KEY(id)
);