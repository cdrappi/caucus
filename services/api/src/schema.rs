table! {
    candidates (slug) {
        slug -> Varchar,
    }
}

table! {
    caucus (state_code) {
        state_code -> Varchar,
        doors_close_utc -> Timestamp,
    }
}

table! {
    counties (slug) {
        slug -> Varchar,
        state_code -> Varchar,
    }
}

table! {
    org_admins (id) {
        id -> Int4,
        user_id -> Int4,
        org -> Varchar,
    }
}

table! {
    orgs (slug) {
        slug -> Varchar,
    }
}

table! {
    precinct_admins (id) {
        id -> Int4,
        user_id -> Int4,
        org -> Varchar,
        precinct -> Varchar,
    }
}

table! {
    precinct_votes (id) {
        id -> Int4,
        candidate -> Varchar,
        precinct -> Varchar,
        alignment -> Int4,
        human_votes -> Int4,
    }
}

table! {
    precincts (slug) {
        slug -> Varchar,
        county_id -> Varchar,
        state_delegates -> Int4,
        attendence -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        is_admin -> Bool,
        username -> Varchar,
        password_hash -> Varchar,
    }
}

joinable!(counties -> caucus (state_code));
joinable!(org_admins -> orgs (org));
joinable!(org_admins -> users (user_id));
joinable!(precinct_admins -> orgs (org));
joinable!(precinct_admins -> precincts (precinct));
joinable!(precinct_admins -> users (user_id));
joinable!(precinct_votes -> candidates (candidate));
joinable!(precinct_votes -> precincts (precinct));
joinable!(precincts -> counties (county_id));

allow_tables_to_appear_in_same_query!(
    candidates,
    caucus,
    counties,
    org_admins,
    orgs,
    precinct_admins,
    precinct_votes,
    precincts,
    users,
);
