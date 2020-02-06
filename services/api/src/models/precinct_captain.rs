use auth::jwt::{decode_jwt, JsonWebToken};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::PgConnection;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use schema::users;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct PrecinctCaptain {
    pub id: i32,
    pub user_id: i32,
    pub precinct: String,
}

impl PrecinctCaptain {
    pub fn can_edit_precinct(user_id: i32, precinct_id: i32) -> bool {
        true
    }
}
