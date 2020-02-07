/// Caucus
use chrono::NaiveDateTime;
use diesel::prelude::PgConnection;
use diesel::result::Error;
use diesel::RunQueryDsl;
use schema::caucus;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Caucus {
    pub state_code: String,
    pub doors_close_utc: NaiveDateTime,
}

impl Caucus {
    pub fn get_caucuses(conn: &PgConnection) -> Result<Vec<Caucus>, Error> {
        return caucus::dsl::caucus.get_results::<Caucus>(conn);
    }
}
