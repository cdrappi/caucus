CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    -- admins can edit everything
    is_admin boolean DEFAULT false NOT NULL,
    username varchar(15) UNIQUE NOT NULL,
    -- and a password, which we hash with bcrypt
    password_hash varchar(64) NOT NULL
);
