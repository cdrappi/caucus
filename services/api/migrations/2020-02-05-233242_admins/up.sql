CREATE TABLE org_admins (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) NOT NULL,
    org varchar(32) REFERENCES orgs(org) NOT NULL
);

CREATE TABLE precinct_admins (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) NOT NULL,
    org varchar(32) REFERENCES orgs(org) NOT NULL,
    precinct varchar(32) REFERENCES precincts(precinct) NOT NULL
);

