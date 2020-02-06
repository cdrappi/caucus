use auth::jwt::JsonWebToken;
use log;
use models::user::User;
use rocket::Route;
use util::JsonResponse;

pub fn get_routes() -> Vec<Route> {
    routes![get_precinct, update_precinct]
}

/// Decode JsonWebToken and return user's ID as JSON
#[get("/caucus/orgs/<org>/precincts/<precinct>", format = "application/json")]
fn get_precinct(
    jwt: JsonWebToken,
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
    "/caucus/orgs/<org>/precincts/<precinct>/update",
    format = "application/json"
)]
fn update_precinct(
    jwt: JsonWebToken,
    org: String,
    precinct: String,
) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!(
        "User {} pinged /caucus/orgs/{}/precincts/{}/update",
        user_id,
        org,
        precinct
    );
    match User::can_edit_votes(user_id, &org, &precinct) {
        true => {
            // TODO: apply edits
            JsonResponse::ok(json!({ "success": true }))
        }
        false => JsonResponse::err400(json!({
            "success": false,
            "error": "Not authorized to edit this precinct"
        })),
    }
}
