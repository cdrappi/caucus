/// Organization Admins
use diesel::dsl::count_star;
use diesel::prelude::PgConnection;
use diesel::result::Error;
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
    pub fn is_org_admin(
        conn: &PgConnection,
        user_id: i32,
        org: &String,
    ) -> bool {
        let org_admin_count_result = org_admins::dsl::org_admins
            .filter(org_admins::dsl::org.eq(org))
            .filter(org_admins::dsl::user_id.eq(user_id))
            .select(count_star())
            .limit(1)
            .get_result(conn);
        let org_admin_count: i32 = org_admin_count_result.expect("Err");
        return org_admin_count > 0;
    }
}
