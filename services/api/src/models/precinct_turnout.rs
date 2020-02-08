/// Precinct
use diesel::prelude::PgConnection;
use diesel::result::Error;
use schema::precinct_turnout_edit_trails;
use schema::precinct_turnouts;

use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct PrecinctTurnout {
    pub id: i32,
    pub edit_trail_id: i32,
    pub org: String,
    pub precinct: String,
    pub turnout: i32,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct PrecinctTurnoutEditTrail {
    pub id: i32,
    pub user_id: i32,
    pub org: String,
    pub precinct: String,
    pub turnout: i32,
}

impl PrecinctTurnout {
    /// edit the turnout value
    pub fn set_turnout(
        conn: &PgConnection,
        user_id: i32,
        precinct: &String,
        turnout: i32,
    ) -> Result<PrecinctTurnout, Error> {
        let new_edit_trail = (
            precinct_turnout_edit_trails::dsl::user_id.eq(user_id),
            precinct_turnout_edit_trails::dsl::precinct.eq(precinct),
            precinct_turnout_edit_trails::dsl::turnout.eq(turnout),
        );
        let edit_trail =
            diesel::insert_into(precinct_turnout_edit_trails::table)
                .values(&new_edit_trail)
                .get_result::<PrecinctTurnoutEditTrail>(conn)
                .expect("Unable to append precinct turnout edit trail");

        return diesel::update(
            precinct_turnouts::dsl::precinct_turnouts
                .filter(precinct_turnouts::dsl::precinct.eq(precinct)),
        )
        .set((
            precinct_turnouts::dsl::turnout.eq(turnout),
            precinct_turnouts::dsl::edit_trail_id.eq(edit_trail.id),
        ))
        .get_result::<PrecinctTurnout>(conn);
    }
}
