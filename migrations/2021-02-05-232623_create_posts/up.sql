-- Your SQL goes here
CREATE TABLE posts
(
    id character varying(27) PRIMARY KEY,
    owner character varying(23) NOT NULL REFERENCES users,
    public BOOLEAN NOT NULL,
    content character varying(500) NOT NULL,
    created timestamp NOT NULL,
    edited timestamp NOT NULL
)