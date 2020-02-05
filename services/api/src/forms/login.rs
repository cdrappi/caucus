/// To login with email, a user must input:
/// - the new desired email number
/// - their password to confirm it's them
#[derive(Serialize, Deserialize)]
pub struct EmailLogin {
    pub email: String,
    pub password: String,
}

/// To login with phone, a user must input:
/// - the new desired phone number
/// - their password to confirm it's them
#[derive(Serialize, Deserialize)]
pub struct PhoneLogin {
    pub phone: String,
    pub password: String,
}

/// To login with username, a user must input:
/// - the new desired username
/// - their password to confirm it's them
#[derive(Serialize, Deserialize)]
pub struct UsernameLogin {
    pub username: String,
    pub password: String,
}
