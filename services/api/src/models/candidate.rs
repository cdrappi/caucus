use auth::jwt::{decode_jwt, JsonWebToken};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::PgConnection;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use schema::users;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub slug: String,
}
