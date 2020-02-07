/// Precinct
use diesel::prelude::PgConnection;
use diesel::result::Error;
use schema::precincts;

use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Precinct {
    pub slug: String,
    pub county_id: String,
    pub state_delegates: i32,
    pub turnout: Option<i32>,
}

impl Precinct {
    pub fn get_county_precincts(
        conn: &PgConnection,
        county_id: &String,
    ) -> Result<Vec<Precinct>, Error> {
        return precincts::dsl::precincts
            .filter(precincts::dsl::county_id.eq(county_id))
            .get_results::<Precinct>(conn);
    }

    /// edit the turnout value
    pub fn set_turnout(
        conn: &PgConnection,
        precinct_slug: &String,
        turnout: i32,
    ) -> Result<Precinct, Error> {
        return diesel::update(
            precincts::dsl::precincts
                .filter(precincts::dsl::slug.eq(precinct_slug)),
        )
        .set(precincts::dsl::turnout.eq(turnout))
        .get_result::<Precinct>(conn);
    }
}
