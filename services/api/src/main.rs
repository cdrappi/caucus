#![feature(proc_macro_hygiene, decl_macro)]
#![feature(never_type)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
extern crate bcrypt;
extern crate chrono;
extern crate jsonwebtoken;
extern crate log;
extern crate reqwest;
extern crate rocket_cors;
extern crate rustc_serialize;

mod auth;
mod db;
mod forms;
mod models;
mod routes;
mod schema;
mod util;
use rocket::http::Method;

use db::DbConn;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use routes::get_routes;

fn main() -> Result<(), Error> {
    let allowed_origins =
        AllowedOrigins::some_exact(&["http://localhost:8080"]);
    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .attach(cors)
        .attach(DbConn::fairing())
        .mount("/", get_routes())
        .launch();

    Ok(())
}
