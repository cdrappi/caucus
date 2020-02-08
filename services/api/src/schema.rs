table! {
    candidates (candidate) {
        candidate -> Varchar,
    }
}

table! {
    caucus (state_code) {
        state_code -> Varchar,
        doors_close_utc -> Timestamp,
    }
}

table! {
    counties (county) {
        county -> Varchar,
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
    orgs (org) {
        org -> Varchar,
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
    precinct_turnout_edit_trails (id) {
        id -> Int4,
        user_id -> Int4,
        org -> Varchar,
        precinct -> Varchar,
        turnout -> Int4,
    }
}

table! {
    precinct_turnouts (id) {
        id -> Int4,
        edit_trail_id -> Int4,
        org -> Varchar,
        precinct -> Varchar,
        turnout -> Int4,
    }
}

table! {
    precinct_vote_edit_trails (id) {
        id -> Int4,
        user_id -> Int4,
        org -> Varchar,
        candidate -> Varchar,
        precinct -> Varchar,
        alignment -> Int4,
        human_votes -> Int4,
    }
}

table! {
    precinct_votes (id) {
        id -> Int4,
        edit_trail_id -> Int4,
        org -> Varchar,
        candidate -> Varchar,
        precinct -> Varchar,
        alignment -> Int4,
        human_votes -> Int4,
    }
}

table! {
    precincts (precinct) {
        precinct -> Varchar,
        county -> Varchar,
        delegates -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        is_admin -> Bool,
        username -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        last_login -> Timestamp,
    }
}

joinable!(counties -> caucus (state_code));
joinable!(org_admins -> orgs (org));
joinable!(org_admins -> users (user_id));
joinable!(precinct_admins -> orgs (org));
joinable!(precinct_admins -> precincts (precinct));
joinable!(precinct_admins -> users (user_id));
joinable!(precinct_turnout_edit_trails -> users (user_id));
joinable!(precinct_turnouts -> precinct_turnout_edit_trails (edit_trail_id));
joinable!(precinct_vote_edit_trails -> candidates (candidate));
joinable!(precinct_vote_edit_trails -> orgs (org));
joinable!(precinct_vote_edit_trails -> precincts (precinct));
joinable!(precinct_vote_edit_trails -> users (user_id));
joinable!(precinct_votes -> candidates (candidate));
joinable!(precinct_votes -> orgs (org));
joinable!(precinct_votes -> precinct_vote_edit_trails (edit_trail_id));
joinable!(precinct_votes -> precincts (precinct));
joinable!(precincts -> counties (county));

allow_tables_to_appear_in_same_query!(
    candidates,
    caucus,
    counties,
    org_admins,
    orgs,
    precinct_admins,
    precinct_turnout_edit_trails,
    precinct_turnouts,
    precinct_vote_edit_trails,
    precinct_votes,
    precincts,
    users,
);
