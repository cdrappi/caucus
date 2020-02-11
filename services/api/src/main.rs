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
use rocket::config::{Config, Environment};
use util::{expect_env, expect_env_u16};

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
use std::collections::HashMap;

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
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    let mut database_config = HashMap::new();
    database_config.insert("url", expect_env("DATABASE_URL"));
    let mut databases = HashMap::new();
    databases.insert("postgres_database", database_config);
    let config = Config::build(Environment::Development)
        .address(expect_env("ROCKET_ADDRESS"))
        // use Heroku's dynamically assigned port
        .port(expect_env_u16("PORT"))
        .workers(5)
        .extra("databases", databases)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .attach(cors)
        .attach(DbConn::fairing())
        .mount("/", get_routes())
        .launch();

    Ok(())
}
