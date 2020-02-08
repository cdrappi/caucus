use auth::jwt::create_jwt;
use db::DbConn;
use forms::register::RegisterUsername;
use log;
use models::user::User;
use rocket::{http::Status, Route};
use rocket_contrib::json::Json;
use util::JsonResponse;

pub fn get_routes() -> Vec<Route> {
    routes![register_with_username,]
}

/// Create a account with username and password (No confirmation)
#[post("/ums/register", format = "application/json", data = "<body>")]
fn register_with_username(
    conn: DbConn,
    body: Json<RegisterUsername>,
) -> JsonResponse {
    return match User::get_from_username(&conn, &body.username) {
        Ok(_) => {
            log::warn!(
                "Tried to create a user with a username that already exists: {}",
                &body.username
            );
            JsonResponse {
                status: Status::UnprocessableEntity,
                json: json!({"success": false, "error": "That user already exists"}),
            }
        }
        Err(_) => {
            // Error is actually good here!
            let user_result = User::get_or_create_from_username(
                &conn,
                &body.username,
                &body.password,
            );
            return match user_result {
                Ok(user) => {
                    log::info!(
                        "Created a user from username {}",
                        &body.username
                    );
                    JsonResponse::ok(
                        json!({"success": true, "jwt": &create_jwt(user.id)}),
                    )
                }
                Err(r) => {
                    log::error!(
                        "Error creating a user with username {}",
                        &body.username
                    );
                    JsonResponse::err500(json!({
                        "success": false,
                        "error": format!("Error get/creating user with username/password: {:}", r)
                    }))
                }
            };
        }
    };
}
