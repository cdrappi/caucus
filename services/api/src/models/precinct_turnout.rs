/// Precinct
use diesel::prelude::PgConnection;
use diesel::result::Error;
use schema::precinct_turnout_edit_trails;
use schema::precinct_turnouts;

use diesel::ExpressionMethods;
use diesel::RunQueryDsl;

#[derive(Queryable, Debug, Serialize, Deserialize, AsChangeset)]
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
        org: &String,
        precinct: &String,
        turnout: i32,
    ) -> Result<PrecinctTurnout, Error> {
        let edit_trail = PrecinctTurnout::create_edit_trail(
            conn, user_id, org, precinct, turnout,
        )
        .expect("Unable to append precinct turnout edit trail");

        let updated_precinct_turnout = (
            precinct_turnouts::dsl::edit_trail_id.eq(edit_trail.id),
            precinct_turnouts::dsl::org.eq(org),
            precinct_turnouts::dsl::precinct.eq(precinct),
            precinct_turnouts::dsl::turnout.eq(turnout),
        );
        return diesel::insert_into(precinct_turnouts::table)
            .values(&updated_precinct_turnout)
            .on_conflict((precinct_turnouts::org, precinct_turnouts::precinct))
            .do_update()
            .set(updated_precinct_turnout)
            .get_result::<PrecinctTurnout>(conn);
    }

    fn create_edit_trail(
        conn: &PgConnection,
        user_id: i32,
        org: &String,
        precinct: &String,
        turnout: i32,
    ) -> Result<PrecinctTurnoutEditTrail, Error> {
        let new_edit_trail = (
            precinct_turnout_edit_trails::dsl::user_id.eq(user_id),
            precinct_turnout_edit_trails::dsl::org.eq(org),
            precinct_turnout_edit_trails::dsl::precinct.eq(precinct),
            precinct_turnout_edit_trails::dsl::turnout.eq(turnout),
        );
        diesel::insert_into(precinct_turnout_edit_trails::table)
            .values(&new_edit_trail)
            .get_result::<PrecinctTurnoutEditTrail>(conn)
    }
}
