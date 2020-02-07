CREATE TABLE orgs (
    slug varchar(32) PRIMARY KEY
);

CREATE TABLE candidates (
    slug varchar(32) PRIMARY KEY
);

CREATE TABLE caucus (
    state_code varchar(2) PRIMARY KEY,
    doors_close_utc timestamp NOT NULL
);

CREATE TABLE counties (
    slug varchar(32) PRIMARY KEY,
    state_code varchar(2) REFERENCES caucus(state_code) NOT NULL
    -- nullables:
);

CREATE TABLE precincts (
    slug varchar(32) PRIMARY KEY,
    county_id varchar(32) REFERENCES counties(slug) NOT NULL,
    state_delegates INT DEFAULT 0 NOT NULL,
    -- nullables:
    turnout INT -- null before results are in
);

CREATE TABLE precinct_votes (
    id SERIAL PRIMARY KEY,
    candidate varchar(32) REFERENCES candidates(slug) NOT NULL,
    precinct varchar(32) references precincts(slug) NOT NULL,
    alignment INT NOT NULL,
    human_votes INT NOT NULL
);
