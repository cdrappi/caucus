use auth::jwt::{decode_jwt, JsonWebToken};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::PgConnection;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use schema::users;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Precinct {
    pub slug: String,
    pub county_id: String,
    pub state_delegates: i32,
    pub attendence: Option<i32>,
}
