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
extern crate rustc_serialize;

mod auth;
mod db;
mod forms;
mod models;
mod routes;
mod schema;
mod util;

use db::DbConn;
use routes::get_routes;

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", get_routes())
        .launch();
}
