use auth::jwt::JsonWebToken;
use bytes::BytesMut;
use db::DbConn;
use forms::user::{UpdateEmail, UpdatePassword, UpdateUsername};
use log;
use models::user::User;
use prost::Message;
use rocket::Route;
use rocket_contrib::json::Json;
use util::JsonResponse;
use util::ProtobufResponse;

pub fn get_user_routes() -> Vec<Route> {
    routes![whose_token_json,]
}

/// Decode JsonWebToken and return user's ID as JSON
#[get("/user/me", format = "application/json", rank = 1)]
fn whose_token_json(jwt: JsonWebToken) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!("User {} pinged the server via json", user_id);
    JsonResponse::ok(json!({ "user_id": user_id }))
}
