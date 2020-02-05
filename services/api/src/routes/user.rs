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
    routes![
        whose_token_json,
        whose_token_protobuf,
        change_password,
        change_username,
        change_email
    ]
}

/// Decode JsonWebToken and return user's ID as JSON
#[get("/user/me", format = "application/json", rank = 1)]
fn whose_token_json(jwt: JsonWebToken) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!("User {} pinged the server via json", user_id);
    JsonResponse::ok(json!({ "user_id": user_id }))
}

/// Decode JsonWebToken and return user's ID as Protobuf
#[get("/user/me", format = "application/protobuf", rank = 2)]
fn whose_token_protobuf(conn: DbConn, jwt: JsonWebToken) -> ProtobufResponse {
    let user_id = User::get_id_from_token(&jwt);
    log::info!("User {} pinged the server via protobuf", user_id);
    let user = User::get_from_id(user_id, &conn).unwrap();
    let msg = user.to_protobuf();
    let mut buf = BytesMut::with_capacity(1024);
    msg.encode(&mut buf).unwrap();
    // user_protobuf.encode_into(&mut data);
    ProtobufResponse::ok(buf.freeze())
}

/// Change password to account
#[post("/user/password", format = "application/json", data = "<body>")]
fn change_password(
    conn: DbConn,
    jwt: JsonWebToken,
    body: Json<UpdatePassword>,
) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    let password_hash = User::hash_password(&body.password);
    let changed_password =
        User::set_password_hash(&user_id, &password_hash, &conn);
    match changed_password {
        Ok(_) => {
            log::info!("Changed password for user {}", user_id);
            JsonResponse::ok(json!({"success": true}))
        }
        Err(e) => {
            log::error!(
                "Failed to change password for user {}: {}",
                user_id,
                e
            );
            JsonResponse::err500(
                json!({"success": false, "error": format!("{:?}", e)}),
            )
        }
    }
}

/// Change username to account
#[post("/user/username", format = "application/json", data = "<body>")]
fn change_username(
    conn: DbConn,
    jwt: JsonWebToken,
    body: Json<UpdateUsername>,
) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    match User::set_username(&user_id, &body.username, &conn) {
        Ok(_) => {
            log::info!(
                "Changed username for user {} to {}",
                user_id,
                &body.username
            );
            JsonResponse::ok(json!({"success": true}))
        }
        Err(e) => {
            log::error!(
                "failed to change username of user {} to {}: {:?}",
                user_id,
                &body.username,
                e
            );
            JsonResponse::err500(
                json!({"success": false, "error": format!("{:?}", e)}),
            )
        }
    }
}

/// Change email to account
#[post("/user/email", format = "application/json", data = "<body>")]
fn change_email(
    conn: DbConn,
    jwt: JsonWebToken,
    body: Json<UpdateEmail>,
) -> JsonResponse {
    let user_id = User::get_id_from_token(&jwt);
    match User::set_email(&user_id, &body.email, &conn) {
        Ok(_) => {
            log::info!("Changed email for user {} to {}", user_id, body.email);
            JsonResponse::ok(json!({"success": true}))
        }
        Err(e) => {
            log::error!(
                "Failed to change email for user {} to {}: {:?}",
                user_id,
                body.email,
                e
            );
            JsonResponse::err500(
                json!({"success": false, "error": format!("{:?}", e)}),
            )
        }
    }
}
