/// Organizations
use schema::orgs;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Org {
    pub slug: String,
}
