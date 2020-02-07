use auth::jwt::JsonWebToken;
use db::DbConn;
use log;
use models::county::County;
use models::user::User;
use rocket::Route;
use util::JsonResponse;

pub fn get_routes() -> Vec<Route> {
    routes![get_county_precincts,]
}

/// Decode JsonWebToken and return user's ID as JSON
#[get("/caucus/states/<state_code>/counties", format = "application/json")]
fn get_county_precincts(
    jwt: JsonWebToken,
    conn: DbConn,
    state_code: String,
) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!(
        "User {} pinged /caucus/states/{}/counties",
        user_id,
        state_code
    );
    match County::get_state_counties(&conn, &state_code) {
        Ok(counties) => JsonResponse::ok(json!({
            "success": true,
            "data": counties
        })),
        Err(e) => JsonResponse::err500(json!({
            "success": false,
            "error": format!("{:?}", e)
        })),
    }
}
