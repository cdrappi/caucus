use auth::jwt::{decode_jwt, JsonWebToken};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::PgConnection;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use protos;
use schema::users;
use util;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub is_admin: bool,
    pub username: Option<String>,
    pub password_hash: String,
}

impl User {
    pub fn to_string(&self) -> String {
        format!("{}", self.id)
    }

    pub fn get_from_id(id: i32, conn: &PgConnection) -> Result<User, Error> {
        return users::dsl::users
            .filter(users::dsl::id.eq(id))
            .get_result::<User>(conn);
    }

    pub fn get_from_username(
        conn: &PgConnection,
        username: &String,
    ) -> Result<User, Error> {
        return users::dsl::users
            .filter(users::dsl::username.eq(username))
            .get_result::<User>(conn);
    }

    fn create_from_username_password(
        conn: &PgConnection,
        username: &String,
        password: &String,
    ) -> Result<usize, Error> {
        let now = Utc::now().naive_utc();
        let new_user = (
            users::dsl::is_admin.eq(false),
            users::dsl::username.eq(username),
            users::dsl::password_hash.eq(User::hash_password(password)),
        );
        return diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn);
    }

    pub fn get_or_create_from_username(
        conn: &PgConnection,
        username: &String,
        password: &String,
    ) -> Result<User, Error> {
        let user_result = users::dsl::users
            .filter(users::dsl::username.eq(username))
            .get_result::<User>(conn);
        match user_result {
            Ok(user) => {
                if user.password_hash == User::hash_password(password) {
                    return Ok(user);
                } else {
                    return Err(Error::NotFound);
                }
            }
            Err(_) => {
                User::create_from_email_password(conn, email, password)?;
                User::get_from_email(conn, email)
            }
        }
    }

    pub fn set_username(
        conn: &PgConnection,
        user_id: &i32,
        username: &String,
    ) -> Result<usize, Error> {
        return diesel::update(
            users::dsl::users.filter(users::dsl::id.eq(user_id)),
        )
        .set(users::dsl::username.eq(username))
        .execute(conn);
    }

    /// Call bcrypt::hash on password with default cost (10^12)
    pub fn hash_password(password: &str) -> String {
        hash(password, DEFAULT_COST).expect("bcrypt::hash returned an error")
    }

    pub fn get_id_from_token(jwt: &JsonWebToken) -> i32 {
        decode_jwt(jwt).expect("Invalid `jwt` argument").claims.sub
    }

    /// Return true iff the input password matches the saved hash
    pub fn validate_password(user: &User, password: &String) -> bool {
        verify(password, &user.password_hash)
            .expect("bcrypt::verify returned an error")
    }

    pub fn set_password_hash(
        conn: &PgConnection,
        user_id: &i32,
        password_hash: &String,
    ) -> Result<usize, Error> {
        return diesel::update(
            users::dsl::users.filter(users::dsl::id.eq(user_id)),
        )
        .set(users::dsl::password_hash.eq(password_hash))
        .execute(conn);
    }
}
