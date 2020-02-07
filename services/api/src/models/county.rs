/// County
use diesel::prelude::PgConnection;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::counties;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct County {
    pub slug: String,
    pub state_code: String,
}

impl County {
    pub fn get_state_counties(
        conn: &PgConnection,
        state_code: &String,
    ) -> Result<Vec<County>, Error> {
        return counties::dsl::counties
            .filter(counties::dsl::state_code.eq(state_code))
            .get_results::<County>(conn);
    }
}
