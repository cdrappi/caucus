/// Precinct voting tallies
use diesel::prelude::PgConnection;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use forms::precinct_vote::NewPrecinctVote;
use schema::precinct_vote_edit_trails;
use schema::precinct_votes;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct PrecinctVote {
    pub id: i32,
    pub edit_trail_id: i32,
    pub org: String,
    pub candidate: String,
    pub precinct: String,
    pub alignment: i32,
    pub human_votes: i32,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct PrecinctEditTrail {
    pub id: i32,
    pub user_id: i32,
    pub org: String,
    pub candidate: String,
    pub precinct: String,
    pub alignment: i32,
    pub human_votes: i32,
}

impl PrecinctVote {
    pub fn create_or_update(
        conn: &PgConnection,
        new_precinct_vote: &NewPrecinctVote,
    ) -> Result<PrecinctVote, Error> {
        // TODO: insert edit trail, get ID
        /*
        diesel::insert_into(precinct_vote_edit_trails::table)
            .values(new_precinct_vote)
            .execute(conn)
        */

        diesel::insert_into(precinct_votes::table)
            .values(new_precinct_vote)
            .on_conflict((
                precinct_votes::org,
                precinct_votes::candidate,
                precinct_votes::precinct,
                precinct_votes::alignment,
            ))
            .do_update()
            .set(new_precinct_vote)
            .get_result(conn)
    }

    pub fn get_votes(
        conn: &PgConnection,
        precinct: &String,
    ) -> Result<Vec<PrecinctVote>, Error> {
        return precinct_votes::dsl::precinct_votes
            .filter(precinct_votes::dsl::precinct.eq(precinct))
            .get_results::<PrecinctVote>(conn);
    }
}
