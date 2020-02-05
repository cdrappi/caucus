/// Since the update routes are guarded by the JsonWebToken
/// request guard, the user only must include their updates
#[derive(Serialize, Deserialize)]
pub struct UpdatePassword {
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUsername {
    pub username: String,
}
#[derive(Serialize, Deserialize)]
pub struct UpdateEmail {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateEmail {
    pub email: String,
    pub password: String,
}
