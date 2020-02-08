use auth::jwt::JsonWebToken;
use db::DbConn;
use forms::precinct_vote::NewPrecinctVote;
use log;
use models::precinct::Precinct;
use models::precinct_turnout::PrecinctTurnout;
use models::precinct_vote::PrecinctVote;
use models::user::User;
use rocket::Route;
use rocket_contrib::json::Json;
use util::JsonResponse;

pub fn get_routes() -> Vec<Route> {
    routes![
        get_county_precincts,
        get_precinct_votes,
        update_precinct_votes,
        update_precinct_turnout
    ]
}

/// Decode JsonWebToken and return user's ID as JSON
#[get("/caucus/counties/<county>/precincts", format = "application/json")]
fn get_county_precincts(
    jwt: JsonWebToken,
    conn: DbConn,
    county: String,
) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!("User {} pinged /caucus/precincts", user_id,);
    match Precinct::get_county_precincts(&conn, &county) {
        Ok(precincts) => JsonResponse::ok(json!({
            "success": true,
            "data": precincts
        })),
        Err(e) => JsonResponse::err500(json!({
            "success": false,
            "error": format!("{:?}", e)
        })),
    }
}

/// Decode JsonWebToken and return user's ID as JSON
#[get("/caucus/orgs/<org>/precincts/<precinct>", format = "application/json")]
fn get_precinct_votes(
    jwt: JsonWebToken,
    conn: DbConn,
    org: String,
    precinct: String,
) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!(
        "User {} pinged /caucus/orgs/{}/precincts/{}",
        user_id,
        org,
        precinct
    );
    match PrecinctVote::get_votes(&conn, &precinct) {
        Ok(precinct_votes) => JsonResponse::ok(json!({
            "success": true,
            "data": precinct_votes
        })),
        Err(e) => JsonResponse::err500(json!({
            "success": false,
            "error": format!("{:?}", e)
        })),
    }
}

/// Update precinct if user is precinct captain or admin
#[post(
    "/caucus/orgs/<org>/precincts/<precinct>/turnout?<turnout>",
    format = "application/json"
)]
fn update_precinct_turnout(
    jwt: JsonWebToken,
    conn: DbConn,
    org: String,
    precinct: String,
    turnout: i32,
) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!(
        "User {} pinged /caucus/orgs/{}/precincts/{}/turnout?turnout={}",
        user_id,
        org,
        precinct,
        turnout
    );
    match User::can_edit_votes(&conn, user_id, &org, &precinct) {
        true => edit_votes(&conn, user_id, &org, &precinct, turnout),
        false => JsonResponse::err400(json!({
            "success": false,
            "error": "Not authorized to edit this precinct"
        })),
    }
}

fn edit_votes(
    conn: &DbConn,
    user_id: i32,
    org: &String,
    precinct: &String,
    turnout: i32,
) -> JsonResponse {
    // TODO: apply edits
    return match PrecinctTurnout::set_turnout(
        conn, user_id, org, precinct, turnout,
    ) {
        Ok(_) => JsonResponse::ok(json!({ "success": true })),
        Err(e) => JsonResponse::err500(
            json!({ "success": false, "error": format!("{:?}", e) }),
        ),
    };
}

/// Update precinct if user is precinct captain or admin
#[post(
    "/caucus/orgs/<org>/precincts/<precinct>/votes",
    format = "application/json",
    data = "<body>"
)]
fn update_precinct_votes(
    jwt: JsonWebToken,
    conn: DbConn,
    org: String,
    precinct: String,
    body: Json<NewPrecinctVote>,
) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!(
        "User {} pinged /caucus/orgs/{}/precincts/{}/update",
        user_id,
        org,
        precinct
    );
    match User::can_edit_votes(&conn, user_id, &org, &precinct) {
        true => {
            // TODO: apply edits
            match PrecinctVote::create_or_update(&conn, user_id, &body) {
                Ok(_) => JsonResponse::ok(json!({ "success": true })),
                Err(e) => JsonResponse::err500(
                    json!({ "success": false, "error": format!("{:?}", e) }),
                ),
            }
        }
        false => JsonResponse::err400(json!({
            "success": false,
            "error": "Not authorized to edit this precinct"
        })),
    }
}
