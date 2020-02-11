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
pub struct PrecinctVoteEditTrail {
    pub id: i32,
    pub user_id: i32,
    pub org: String,
    pub candidate: String,
    pub precinct: String,
    pub alignment: i32,
    pub human_votes: i32,
}

impl PrecinctVoteEditTrail {
    fn create(
        conn: &PgConnection,
        user_id: i32,
        vote: &NewPrecinctVote,
    ) -> Result<PrecinctVoteEditTrail, Error> {
        let new_edit_trail = (
            precinct_vote_edit_trails::dsl::user_id.eq(user_id),
            precinct_vote_edit_trails::dsl::org.eq(&vote.org),
            precinct_vote_edit_trails::dsl::candidate.eq(&vote.candidate),
            precinct_vote_edit_trails::dsl::precinct.eq(&vote.precinct),
            precinct_vote_edit_trails::dsl::alignment.eq(vote.alignment),
            precinct_vote_edit_trails::dsl::human_votes.eq(vote.human_votes),
        );
        diesel::insert_into(precinct_vote_edit_trails::table)
            .values(&new_edit_trail)
            .get_result::<PrecinctVoteEditTrail>(conn)
    }
}

#[derive(Queryable, Debug, Serialize, Deserialize, AsChangeset)]
pub struct PrecinctVote {
    pub id: i32,
    pub edit_trail_id: i32,
    pub org: String,
    pub candidate: String,
    pub precinct: String,
    pub alignment: i32,
    pub human_votes: i32,
}

impl PrecinctVote {
    pub fn create_or_update(
        conn: &PgConnection,
        user_id: i32,
        vote: &NewPrecinctVote,
    ) -> Result<PrecinctVote, Error> {
        let edit_trail = PrecinctVoteEditTrail::create(conn, user_id, vote)
            .expect("Unable to append precinct vote edit trail");

        let updated_precinct_vote = (
            precinct_votes::dsl::edit_trail_id.eq(edit_trail.id),
            precinct_votes::dsl::org.eq(&vote.org),
            precinct_votes::dsl::candidate.eq(&vote.candidate),
            precinct_votes::dsl::precinct.eq(&vote.precinct),
            precinct_votes::dsl::alignment.eq(vote.alignment),
            precinct_votes::dsl::human_votes.eq(vote.human_votes),
        );
        return diesel::insert_into(precinct_votes::table)
            .values(&updated_precinct_vote)
            .on_conflict((
                precinct_votes::org,
                precinct_votes::candidate,
                precinct_votes::precinct,
                precinct_votes::alignment,
            ))
            .do_update()
            .set(updated_precinct_vote)
            .get_result::<PrecinctVote>(conn);
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
