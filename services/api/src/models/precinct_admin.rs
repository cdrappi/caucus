/// Precinct admins
// use schema::precinct_admins;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct PrecinctAdmin {
    pub id: i32,
    pub user_id: i32,
    pub org: String,
    pub precinct: String,
}
