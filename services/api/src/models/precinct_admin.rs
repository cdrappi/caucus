/// Precinct admins
use diesel::dsl::count_star;
use diesel::prelude::PgConnection;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::precinct_admins;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct PrecinctAdmin {
    pub id: i32,
    pub user_id: i32,
    pub org: String,
    pub precinct: String,
}

impl PrecinctAdmin {
    /// See if user is admin for this specific precinct
    pub fn is_precinct_admin(
        conn: &PgConnection,
        user_id: i32,
        org: &String,
        precinct: &String,
    ) -> bool {
        let precinct_admin_count_result = precinct_admins::table
            .select(count_star())
            .filter(precinct_admins::dsl::user_id.eq(user_id))
            .filter(precinct_admins::dsl::org.eq(org))
            .filter(precinct_admins::dsl::precinct.eq(precinct))
            .first::<i64>(conn);
        return match precinct_admin_count_result {
            Ok(precinct_admin_count) => precinct_admin_count > 0,
            Err(_) => false,
        };
    }
}
