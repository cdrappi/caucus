use reqwest::{Client, Error, Response};
use util::expect_env;

fn get_env() -> (String, String, String) {
    return (
        expect_env("TWILIO_SERVICE"),
        expect_env("TWILIO_ACCOUNT_SID"),
        expect_env("TWILIO_AUTH_TOKEN"),
    );
}

pub fn post_phone(phone: &str) -> Result<Response, Error> {
    let client = Client::new();
    let (service, account_sid, token) = get_env();
    let url = format!(
        "https://verify.twilio.com/v2/Services/{}/Verifications",
        service
    );
    client
        .post(&url)
        .basic_auth(account_sid, Some(token))
        .form(&[("Channel", "sms"), ("To", &format!("+{}", phone))])
        .send()
}

pub fn post_verification(phone: &str, code: &str) -> Result<Response, Error> {
    let client = Client::new();
    let (service, account_sid, token) = get_env();
    let url = format!(
        "https://verify.twilio.com/v2/Services/{}/VerificationCheck",
        service
    );
    client
        .post(&url)
        .basic_auth(account_sid, Some(token))
        .form(&[("To", &format!("+{}", phone)), ("Code", &code.to_string())])
        .send()
}
