use auth::jwt::JsonWebToken;
use log;
use models::precinct_captain::PrecinctCaptain;
use models::user::User;
use rocket::Route;
use util::JsonResponse;

pub fn get_routes() -> Vec<Route> {
    routes![get_precinct, update_precinct]
}

/// Decode JsonWebToken and return user's ID as JSON
#[get("/caucus/precincts/<precinct_id>", format = "application/json")]
fn get_precinct(jwt: JsonWebToken, precinct_id: i32) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!("User {} pinged /caucus/precincts/{}", user_id, precinct_id);
    JsonResponse::ok(json!({ "success": true, "data": {"id": precinct_id} }))
}

/// Update precinct if user is precinct captain or admin
#[post("/caucus/precincts/<precinct_id>/update", format = "application/json")]
fn update_precinct(jwt: JsonWebToken, precinct_id: i32) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!(
        "User {} pinged /caucus/precincts/{}/update",
        user_id,
        precinct_id
    );
    match PrecinctCaptain::can_edit_precinct(user_id, precinct_id) {
        true => {
            // apply edits
            JsonResponse::ok(json!({ "success": true }))
        }
        false => JsonResponse::err400(json!({
            "success": false,
            "error": "Not authorized to edit this precinct"
        })),
    }
}
