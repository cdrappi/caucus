/// Precinct
// use schema::precincts;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Precinct {
    pub slug: String,
    pub county_id: String,
    pub state_delegates: i32,
    pub attendence: Option<i32>,
}
