/// To login with username, a user must input:
/// - the new desired username
/// - their password to confirm it's them
#[derive(Serialize, Deserialize)]
pub struct UsernameLogin {
    pub username: String,
    pub password: String,
}
