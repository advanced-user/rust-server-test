-- Add migration script here
CREATE TABLE employee (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL
);
