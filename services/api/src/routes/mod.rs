pub mod login;
pub mod register;
pub mod user;

use db::DbConn;
use rocket::Route;

#[get("/", format = "text/html")]
fn index(_conn: DbConn) -> String {
    "hello, world".to_string()
}

pub fn get_routes() -> Vec<Route> {
    let mut route_vec = routes![index];
    route_vec.extend(register::get_register_routes().iter().cloned());
    route_vec.extend(login::get_login_routes().iter().cloned());
    route_vec.extend(user::get_user_routes().iter().cloned());
    route_vec
}
