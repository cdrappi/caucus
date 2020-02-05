use auth::jwt::create_jwt;
use auth::twilio;
use db::DbConn;
use forms::register::RegisterUsername;
use forms::twilio::{SubmitPhone, VerifyPhone};
use log;
use models::user::User;
use reqwest::StatusCode;
use rocket::{http::Status, Route};
use rocket_contrib::json::Json;
use util::JsonResponse;

pub fn get_register_routes() -> Vec<Route> {
    routes![register_with_username,]
}

/// Create a account with email and password (No confirmation)
#[post("/register/username", format = "application/json", data = "<body>")]
fn register_with_username(
    conn: DbConn,
    body: Json<RegisterUsername>,
) -> JsonResponse {
    return match User::get_from_username(&conn, &body.username) {
        Ok(_) => {
            log::warn!(
                "Tried to create a user with an email that already exists: {}",
                &body.email
            );
            JsonResponse {
                status: Status::UnprocessableEntity,
                json: json!({"success": false, "error": "That user already exists"}),
            }
        }
        Err(_) => {
            // Error is actually good here!
            let user_result = User::get_or_create_from_email(
                &body.email,
                &body.password,
                &conn,
            );
            return match user_result {
                Ok(user) => {
                    log::info!("Created a user from email {}", &body.email);
                    JsonResponse::ok(
                        json!({"success": true, "jwt": &create_jwt(user.id)}),
                    )
                }
                Err(r) => {
                    log::error!(
                        "Error creating a user with email {}",
                        &body.email
                    );
                    JsonResponse::err500(json!({
                        "success": false,
                        "error": format!("Error get/creating user with email/password: {:}", r)
                    }))
                }
            };
        }
    };
}
