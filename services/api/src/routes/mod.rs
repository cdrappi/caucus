pub mod caucus;
pub mod counties;
pub mod login;
pub mod precincts;
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
    route_vec.extend(register::get_routes().iter().cloned());
    route_vec.extend(login::get_routes().iter().cloned());
    route_vec.extend(user::get_routes().iter().cloned());
    route_vec.extend(caucus::get_routes().iter().cloned());
    route_vec.extend(counties::get_routes().iter().cloned());
    route_vec.extend(precincts::get_routes().iter().cloned());
    route_vec
}
