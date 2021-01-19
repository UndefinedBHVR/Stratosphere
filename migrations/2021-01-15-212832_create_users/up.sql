-- Your SQL goes here
CREATE TABLE users
(
    id character varying(23) NOT NULL PRIMARY KEY,
    nickname character varying(32) NOT NULL,
    email character varying(191) UNIQUE NOT NULL,
    password character varying(191) NOT NULL,
    rank integer NOT NULL,
    is_priv boolean NOT NULL,
    updated_at timestamp NOT NULL,
    created_at timestamp NOT NULL
)