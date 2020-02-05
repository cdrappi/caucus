use auth::jwt;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, Header, TokenData,
};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use util::{expect_env, now_seconds, JsonResponse};

pub fn token_ok(user_id: i32) -> JsonResponse {
    JsonResponse::ok(json!({"success": true,"jwt": create_jwt(user_id)}))
}

/// JSON Web Token claims contains two fields:
///
/// - `sub` (user id)
/// - `exp` (expiration timestamp)
#[derive(Debug, Serialize, Deserialize, RustcEncodable, RustcDecodable)]
pub struct JwtClaims {
    /// the `sub` field receives the User's id
    pub sub: i32,
    pub exp: u64,
}

/// Wrap call to `decode` in the `jsonwebtoken` crate
pub fn decode_jwt(jwt: &JsonWebToken) -> Result<TokenData<JwtClaims>, Error> {
    let secret = expect_env("ROCKET_SECRET_KEY");
    let token_data =
        decode::<JwtClaims>(&jwt.0, secret.as_ref(), Algorithm::HS256);
    return token_data;
}

pub fn create_jwt(user_id: i32) -> String {
    let claims = jwt::JwtClaims {
        sub: user_id,
        exp: now_seconds(),
    };
    let secret = expect_env("ROCKET_SECRET_KEY");
    return encode(Header::default(), &claims, secret.as_bytes())
        .expect("Error creating JWT");
}

/// `JsonWebToken`
///
/// A request guard that passes if we find and decode
/// a JSON Web Token in the `"Authorization"` field of request headers
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonWebToken(String);

/// JWT auth can fail in 3 cases:
///
/// - User did not provide any token
/// - User provided one invalid token
/// - User provided more than one token
#[derive(Debug)]
pub enum JwtAuthError {
    Missing,
    Invalid,
    TooMany,
}

impl<'a, 'r> FromRequest<'a, 'r> for JsonWebToken {
    type Error = JwtAuthError;

    /// Request guard that validates a JsonWebToken
    fn from_request(
        request: &'a Request<'r>,
    ) -> Outcome<JsonWebToken, JwtAuthError> {
        let tokens: Vec<_> = request.headers().get("Authorization").collect();
        match tokens.len() {
            0 => Outcome::Failure((Status::BadRequest, JwtAuthError::Missing)),
            1 => {
                let jwt = JsonWebToken(tokens[0].to_string());
                match decode_jwt(&jwt) {
                    Ok(_) => Outcome::Success(jwt),
                    Err(_) => Outcome::Failure((
                        Status::BadRequest,
                        JwtAuthError::Invalid,
                    )),
                }
            }
            _ => Outcome::Failure((Status::BadRequest, JwtAuthError::TooMany)),
        }
    }
}
