CREATE TABLE precinct_captains (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),
    precinct varchar(32) REFERENCES precincts(slug)
)