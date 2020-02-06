/// To create a new user, input:
/// - the new desired username
/// - their password to confirm it's them
#[derive(Serialize, Deserialize)]
pub struct RegisterUsername {
    pub username: String,
    pub password: String,
}
