/// NewPrecinctVote report results for a single candidate
use schema::precinct_votes;

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "precinct_votes"]
pub struct NewPrecinctVote {
    pub org: String,
    pub candidate: String,
    pub precinct: String,
    pub alignment: i32,
    pub human_votes: i32,
}
