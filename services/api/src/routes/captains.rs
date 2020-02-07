use auth::jwt::JsonWebToken;
use log;
use models::user::User;
use rocket::Route;
use util::JsonResponse;

pub fn get_routes() -> Vec<Route> {
    routes![make_captain]
}

/// Decode JsonWebToken and return user's ID as JSON
#[post(
    "/caucus/precincts/<precinct_id>/captains/<username>/add",
    format = "application/json"
)]
fn add_precinct_captain(jwt: JsonWebToken, precinct_id: i32) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!("User {} pinged /caucus/precincts/{}", user_id, precinct_id);
    JsonResponse::ok(json!({ "success": true, "data": {"id": precinct_id} }))
}
