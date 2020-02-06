use auth::jwt::JsonWebToken;
use db::DbConn;
use forms::precinct_vote::NewPrecinctVote;
use log;
use models::precinct::Precinct;
use models::precinct_vote::PrecinctVote;
use models::user::User;
use rocket::Route;
use rocket_contrib::json::Json;
use util::JsonResponse;

pub fn get_routes() -> Vec<Route> {
    routes![get_precinct, update_precinct_votes, update_precinct_turnout]
}

/// Decode JsonWebToken and return user's ID as JSON
#[get("/caucus/orgs/<org>/precincts/<precinct>", format = "application/json")]
fn get_precinct(
    jwt: JsonWebToken,
    conn: DbConn,
    org: String,
    precinct: i32,
) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!(
        "User {} pinged /caucus/orgs/{}/precincts/{}",
        user_id,
        org,
        precinct
    );
    JsonResponse::ok(
        json!({ "success": true, "data": {"precinct": precinct} }),
    )
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
        true => {
            // TODO: apply edits
            match Precinct::set_turnout(&conn, &precinct, turnout) {
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
            match PrecinctVote::create_or_update(&conn, &body) {
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
