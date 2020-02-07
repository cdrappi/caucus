use auth::jwt::JsonWebToken;
use db::DbConn;
use log;
use models::caucus::Caucus;
use models::user::User;
use rocket::Route;
use util::JsonResponse;

pub fn get_routes() -> Vec<Route> {
    routes![get_caucuses,]
}

/// Decode JsonWebToken and return user's ID as JSON
#[get("/caucus/states", format = "application/json")]
fn get_caucuses(jwt: JsonWebToken, conn: DbConn) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!("User {} pinged /caucus/states", user_id);
    match Caucus::get_caucuses(&conn) {
        Ok(caucuses) => JsonResponse::ok(json!({
            "success": true,
            "data": caucuses
        })),
        Err(e) => JsonResponse::err500(json!({
            "success": false,
            "error": format!("{:?}", e)
        })),
    }
}
