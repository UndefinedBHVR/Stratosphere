-- Your SQL goes here
CREATE TABLE auths
(
    token character varying(25) NOT NULL UNIQUE,
    refresh character varying(33) NOT NULL PRIMARY KEY,
    owner character varying(23) NOT NULL REFERENCES users,
    expiry timestamp NOT NULL,
    created timestamp NOT NULL
)