-- Your SQL goes here

CREATE TABLE cities (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    population INT NOT NULL,
    country VARCHAR NOT NULL
)