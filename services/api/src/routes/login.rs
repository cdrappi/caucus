use auth::jwt::token_ok;
use db::DbConn;
use diesel::result::Error;
use forms::login::{EmailLogin, PhoneLogin, UsernameLogin};
use log;
use models::user::User;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;
use util::JsonResponse;

pub fn get_login_routes() -> Vec<Route> {
    routes![login_with_phone, login_with_email, login_with_username,]
}

/// Login using phone number and password
#[post("/login/phone", format = "application/json", data = "<body>")]
fn login_with_phone(conn: DbConn, body: Json<PhoneLogin>) -> JsonResponse {
    let user_result = User::get_from_phone(&body.phone, &conn);
    login_user_result(user_result, "phone", &body.phone, &body.password)
}

/// Login using email address and password
#[post("/login/email", format = "application/json", data = "<body>")]
fn login_with_email(conn: DbConn, body: Json<EmailLogin>) -> JsonResponse {
    let user_result = User::get_from_email(&body.email, &conn);
    login_user_result(user_result, "email", &body.email, &body.password)
}

/// Login with usernane and password
#[post("/login/username", format = "application/json", data = "<body>")]
fn login_with_username(
    conn: DbConn,
    body: Json<UsernameLogin>,
) -> JsonResponse {
    let user_result = User::get_from_username(&body.username, &conn);
    login_user_result(user_result, "username", &body.username, &body.password)
}

fn login_user(user: &User, password: &String) -> JsonResponse {
    log::info!("Logged in user {}", user.to_string());
    match User::validate_password(user, password) {
        true => token_ok(user.id),
        false => JsonResponse {
            status: Status::Unauthorized,
            json: json!({"success": false}),
        },
    }
}

fn error_logging_in(
    login_type: &str,
    login: &String,
    e: diesel::result::Error,
) -> JsonResponse {
    log::error!("User login error: {} {}", login_type, login);
    JsonResponse {
        status: Status::NotFound,
        json: json!({
            "success": false,
            "error": format!("Error finding user with {} {}: {}", login_type, login, e)
        }),
    }
}

fn login_user_result(
    user_result: Result<User, Error>,
    login_type: &str,
    login: &String,
    password: &String,
) -> JsonResponse {
    match user_result {
        Ok(user) => login_user(&user, password),
        Err(e) => error_logging_in(login_type, login, e),
    }
}
