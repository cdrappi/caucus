/// County

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct County {
    pub slug: String,
    pub state_code: String,
}
