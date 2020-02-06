use auth::jwt::JsonWebToken;
use db::DbConn;
use log;
use models::user::User;
use rocket::Route;
use util::JsonResponse;

pub fn get_routes() -> Vec<Route> {
    routes![whose_token_json, make_user_admin]
}

/// Decode JsonWebToken and return user's ID as JSON
#[get("/ums/me", format = "application/json", rank = 1)]
fn whose_token_json(jwt: JsonWebToken) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!("User {} pinged the server via json", user_id);
    JsonResponse::ok(json!({ "user_id": user_id }))
}

/// Decode JsonWebToken and return user's ID as JSON
#[post("/ums/users/<username>/make_admin", format = "application/json")]
fn make_user_admin(
    conn: DbConn,
    jwt: JsonWebToken,
    username: String,
) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!("User {} pinged /ums/users/{}/make_admin", user_id, username);
    match User::get_from_id(&conn, user_id) {
        Ok(user) => match user.is_admin {
            true => match User::get_from_username(&conn, &username) {
                Ok(new_admin) => {
                    User::set_admin(&conn, new_admin.id, true);
                    JsonResponse::ok(json!({"success": true }))
                }
                Err(_) => JsonResponse::err400(json!({
                        "success": false,
                        "error": "cannot get user with that username"
                })),
            },
            false => JsonResponse::err400(json!({
                "success": false,
                "error": "Only admins can make other users admins"
            })),
        },
        Err(_) => JsonResponse::err500(json!({"success": false})),
    }
}
