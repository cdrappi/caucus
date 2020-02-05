use auth::jwt::create_jwt;
use auth::twilio;
use db::DbConn;
use forms::twilio::{SubmitPhone, VerifyPhone};
use forms::user::CreateEmail;
use log;
use models::user::User;
use reqwest::StatusCode;
use rocket::{http::Status, Route};
use rocket_contrib::json::Json;
use util::JsonResponse;

pub fn get_register_routes() -> Vec<Route> {
    routes![
        register_submit_phone_number,
        register_submit_phone_code,
        register_with_email,
    ]
}

/// Create a account with email and password (No confirmation)
#[post("/register/email", format = "application/json", data = "<body>")]
fn register_with_email(conn: DbConn, body: Json<CreateEmail>) -> JsonResponse {
    return match User::get_from_email(&body.email, &conn) {
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

/// Ask us to ask Twilio to send you an SMS code
#[post("/register/phone", format = "application/json", data = "<body>")]
fn register_submit_phone_number(body: Json<SubmitPhone>) -> JsonResponse {
    let response = twilio::post_phone(&body.phone);
    return match response {
        Ok(_) => {
            log::info!("Submitted phone {} to Twilio", &body.phone);
            JsonResponse {
                status: Status::Ok,
                json: json!({"success": true, "phone": &body.phone}),
            }
        }
        Err(e) => {
            log::error!(
                "Threw error submitting phone {} to Twilio: {}",
                &body.phone,
                e,
            );
            JsonResponse::err400(
                json!({"success": false, "error": format!("{:?}", e)}),
            )
        }
    };
}

/// Verify your phone number via Twilio SMS code
#[post("/register/verify", format = "application/json", data = "<body>")]
fn register_submit_phone_code(
    conn: DbConn,
    body: Json<VerifyPhone>,
) -> JsonResponse {
    let resp_result = twilio::post_verification(&body.phone, &body.code);
    return match resp_result {
        Ok(response) => match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                log::info!(
                    "Twilio responded Ok to phone {} with code {}",
                    &body.phone,
                    &body.code
                );
                let user_result = User::get_or_create_from_phone(
                    &body.phone,
                    &body.code,
                    &conn,
                );
                return match user_result {
                    Ok(user) => {
                        log::info!(
                            "Created new user with phone {}",
                            &body.phone
                        );
                        JsonResponse::ok(
                            json!({"success": true, "jwt": create_jwt(user.id)}),
                        )
                    }
                    Err(e) => {
                        log::error!(
                            "Unknown error creating user with phone {}: {:?}",
                            &body.phone,
                            e
                        );
                        JsonResponse::err500(json!({
                            "success": false,
                            "error": format!("Error retrieving user from phone {}: {:?}", body.phone, e)
                        }))
                    }
                };
            }
            _ => {
                log::error!(
                    "Twilio responded with invalid code: {}",
                    response.status()
                );
                JsonResponse::err500(json!({"success": false,
                "error": format!("Twilio responded with invalid code: {}", response.status())}))
            }
        },
        Err(r) => {
            log::error!("Error posting verification to Twilio: {:}", r);
            JsonResponse::err500(json!({
                "success": false,
                "error": format!("Error posting verification to Twilio: {:}", r)
            }))
        }
    };
}
