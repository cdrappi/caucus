/// Precinct
use diesel::prelude::PgConnection;
use diesel::result::Error;
use schema::precincts;

use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Precinct {
    pub precinct: String,
    pub county: String,
    pub delegates: i32,
    pub turnout: Option<i32>,
}

impl Precinct {
    pub fn get_county_precincts(
        conn: &PgConnection,
        county: &String,
    ) -> Result<Vec<Precinct>, Error> {
        return precincts::dsl::precincts
            .filter(precincts::dsl::county.eq(county))
            .get_results::<Precinct>(conn);
    }

    /// edit the turnout value
    pub fn set_turnout(
        conn: &PgConnection,
        precinct: &String,
        turnout: i32,
    ) -> Result<Precinct, Error> {
        return diesel::update(
            precincts::dsl::precincts
                .filter(precincts::dsl::precinct.eq(precinct)),
        )
        .set(precincts::dsl::turnout.eq(turnout))
        .get_result::<Precinct>(conn);
    }
}
