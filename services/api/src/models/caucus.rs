/// Caucus
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Caucus {
    pub state_code: String,
    pub doors_close_utc: NaiveDateTime,
}
