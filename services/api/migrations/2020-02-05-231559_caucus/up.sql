CREATE TABLE orgs (
    org varchar(32) PRIMARY KEY
);

CREATE TABLE candidates (
    candidate varchar(32) PRIMARY KEY
);

CREATE TABLE caucus (
    state_code varchar(2) PRIMARY KEY,
    doors_close_utc timestamp NOT NULL
);

CREATE TABLE counties (
    county varchar(32) PRIMARY KEY,
    state_code varchar(2) REFERENCES caucus(state_code) NOT NULL,
    delegates INT NOT NUll
    -- nullables:
);

CREATE TABLE precincts (
    precinct varchar(32) PRIMARY KEY,
    county varchar(32) REFERENCES counties(county) NOT NULL,
    delegates INT NOT NULL
);

CREATE TABLE precinct_turnout_edit_trails (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) NOT NULL,
    org varchar(32) NOT NULL,
    precinct varchar(32) NOT NULL,
    turnout INT NOT NULL -- null before results are in
);


CREATE TABLE precinct_vote_edit_trails (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) NOT NULL,
    org varchar(32) REFERENCES orgs(org) NOT NULL,
    candidate varchar(32) REFERENCES candidates(candidate) NOT NULL,
    precinct varchar(32) REFERENCES precincts(precinct) NOT NULL,
    alignment INT NOT NULL,
    human_votes INT NOT NULL
);

CREATE TABLE precinct_turnouts (
    id SERIAL PRIMARY KEY,
    -- where it came from
    edit_trail_id INT REFERENCES precinct_turnout_edit_trails(id) NOT NULL,
    -- unique on
    org varchar(32) NOT NULL,
    precinct varchar(32) NOT NULL,
    -- the data
    turnout INT NOT NULL, -- null before results are in
    UNIQUE(org, precinct)
);


CREATE TABLE precinct_votes (
    id SERIAL PRIMARY KEY,
    -- trace of where it came from
    edit_trail_id INT REFERENCES precinct_vote_edit_trails(id) NOT NULL,
    -- unique on
    org varchar(32) REFERENCES orgs(org) NOT NULL,
    candidate varchar(32) REFERENCES candidates(candidate) NOT NULL,
    precinct varchar(32) REFERENCES precincts(precinct) NOT NULL,
    -- actual data
    alignment INT NOT NULL,
    human_votes INT NOT NULL,
    -- unique constraint
    UNIQUE(org, candidate, precinct) -- each org can overwrite each other
);

