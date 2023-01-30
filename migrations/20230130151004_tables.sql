-- Add migration script here
CREATE DATABASE IF NOT EXISTS persona;

CREATE TABLE persona.personas (
    id string PRIMARY KEY,
    identifiers blob NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    parent_id string
    FOREIGN KEY (parent_id) REFERENCES persona.personas(id)
)

CREATE TABLE persona.persona_identifiers (
    identifier string PRIMARY KEY UNIQUE,
    persona_id string NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    UNIQUE (persona_id, identifier)
    FOREIGN KEY (persona_id) REFERENCES persona.personas(id)
)