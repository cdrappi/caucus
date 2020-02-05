/// First step of registration: enter your phone number
#[derive(Serialize, Deserialize)]
pub struct SubmitPhone {
    pub phone: String,
}

/// To verify phone, input phone and verification code
#[derive(Serialize, Deserialize)]
pub struct VerifyPhone {
    pub phone: String,
    pub code: String,
}
