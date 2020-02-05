DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    -- all users must have a verified phone number
    -- No leading "+", e.g. "18008675309"
    created_at timestamp NOT NULL,
    last_login timestamp NOT NULL,
    -- and a password, which we hash to 32 characters
    password_hash varchar(64) NOT NULL,
    -- later they can add email address & username
    phone varchar(15) UNIQUE,
    email varchar(320) UNIQUE,
    username varchar(15) UNIQUE
);
CREATE UNIQUE INDEX uphone ON users(phone);
CREATE UNIQUE INDEX uemail ON users(email);
CREATE UNIQUE INDEX uname ON users(username);
