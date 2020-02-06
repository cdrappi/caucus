/// Organization Admins
use diesel::dsl::count_star;
use diesel::prelude::PgConnection;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::org_admins;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct OrgAdmin {
    pub id: i32,
    pub user_id: i32,
    pub org: String,
}

impl OrgAdmin {
    /// See if this user is admin for this org
    pub fn is_org_admin(
        conn: &PgConnection,
        user_id: i32,
        org: &String,
    ) -> bool {
        let org_admin_count_result = org_admins::table
            .select(count_star())
            .filter(org_admins::dsl::user_id.eq(user_id))
            .filter(org_admins::dsl::org.eq(org))
            .first::<i64>(conn);
        return match org_admin_count_result {
            Ok(org_admin_count) => org_admin_count > 0,
            Err(_) => false,
        };
    }
}
