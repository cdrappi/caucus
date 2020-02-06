/// Precinct voting tallies
use diesel::prelude::PgConnection;
use diesel::result::Error;
use diesel::RunQueryDsl;
use forms::precinct_vote::NewPrecinctVote;
use schema::precinct_votes;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct PrecinctVote {
    pub id: i32,
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
        diesel::insert_into(precinct_votes::table)
            .values(new_precinct_vote)
            .on_conflict((
                precinct_votes::candidate,
                precinct_votes::precinct,
                precinct_votes::alignment,
            ))
            .do_update()
            .set(new_precinct_vote)
            .get_result(conn)
    }
}
