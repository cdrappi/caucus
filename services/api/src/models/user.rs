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
    pub created_at: NaiveDateTime,
    pub last_login: NaiveDateTime,
    pub password_hash: String,
    pub phone: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}

impl User {
    /// Serialize a User struct to protobuf
    pub fn to_protobuf(&self) -> protos::user::User {
        let mut user_protobuf = protos::user::User::default();
        user_protobuf.id = self.id;
        user_protobuf.created_at =
            protos::time::chrono_to_prost(&self.created_at);
        user_protobuf.last_login =
            protos::time::chrono_to_prost(&self.last_login);
        user_protobuf.phone = util::to_string(&self.phone);
        user_protobuf.email = util::to_string(&self.email);
        user_protobuf.username = util::to_string(&self.username);
        return user_protobuf;
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.id)
    }

    pub fn get_from_id(id: i32, conn: &PgConnection) -> Result<User, Error> {
        return users::dsl::users
            .filter(users::dsl::id.eq(id))
            .get_result::<User>(conn);
    }

    pub fn get_from_phone(
        phone: &String,
        conn: &PgConnection,
    ) -> Result<User, Error> {
        return users::dsl::users
            .filter(users::dsl::phone.eq(phone))
            .get_result::<User>(conn);
    }

    pub fn get_from_email(
        email: &String,
        conn: &PgConnection,
    ) -> Result<User, Error> {
        return users::dsl::users
            .filter(users::dsl::email.eq(email))
            .get_result::<User>(conn);
    }

    pub fn get_from_username(
        username: &String,
        conn: &PgConnection,
    ) -> Result<User, Error> {
        return users::dsl::users
            .filter(users::dsl::username.eq(username))
            .get_result::<User>(conn);
    }

    fn create_from_phone(
        phone: &String,
        code: &String,
        conn: &PgConnection,
    ) -> Result<usize, Error> {
        let now = Utc::now().naive_utc();
        let new_user = (
            users::dsl::phone.eq(phone),
            users::dsl::password_hash.eq(User::hash_password(code)),
            users::dsl::created_at.eq(now),
            users::dsl::last_login.eq(now),
        );
        return diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn);
    }

    fn create_from_email_password(
        email: &String,
        password: &String,
        conn: &PgConnection,
    ) -> Result<usize, Error> {
        let now = Utc::now().naive_utc();
        let new_user = (
            users::dsl::email.eq(email),
            users::dsl::password_hash.eq(User::hash_password(password)),
            users::dsl::created_at.eq(now),
            users::dsl::last_login.eq(now),
        );
        return diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn);
    }

    pub fn get_or_create_from_phone(
        phone: &String,
        code: &String,
        conn: &PgConnection,
    ) -> Result<User, Error> {
        let user_result = users::dsl::users
            .filter(users::dsl::phone.eq(phone))
            .get_result::<User>(conn);
        match user_result {
            Ok(user) => {
                User::set_last_login(user.id, conn).unwrap();
                Ok(user)
            }
            Err(_) => {
                User::create_from_phone(phone, code, conn)?;
                User::get_from_phone(phone, conn)
            }
        }
    }

    pub fn get_or_create_from_email(
        email: &String,
        password: &String,
        conn: &PgConnection,
    ) -> Result<User, Error> {
        let user_result = users::dsl::users
            .filter(users::dsl::email.eq(email))
            .get_result::<User>(conn);
        match user_result {
            Ok(user) => {
                if user.password_hash == User::hash_password(password) {
                    User::set_last_login(user.id, conn).unwrap();
                    return Ok(user);
                } else {
                    return Err(Error::NotFound);
                }
            }
            Err(_) => {
                User::create_from_email_password(email, password, conn)?;
                User::get_from_email(email, conn)
            }
        }
    }

    pub fn set_username(
        user_id: &i32,
        username: &String,
        conn: &PgConnection,
    ) -> Result<usize, Error> {
        return diesel::update(
            users::dsl::users.filter(users::dsl::id.eq(user_id)),
        )
        .set(users::dsl::username.eq(username))
        .execute(conn);
    }

    pub fn set_email(
        user_id: &i32,
        email: &String,
        conn: &PgConnection,
    ) -> Result<usize, Error> {
        return diesel::update(
            users::dsl::users.filter(users::dsl::id.eq(user_id)),
        )
        .set(users::dsl::email.eq(email))
        .execute(conn);
    }

    fn set_last_login(id: i32, conn: &PgConnection) -> Result<usize, Error> {
        let now = Utc::now().naive_utc();
        return diesel::update(
            users::dsl::users.filter(users::dsl::id.eq(id)),
        )
        .set(users::dsl::last_login.eq(now))
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
        verify(password, &user.password_hash).expect("bcrypt::verify returned an error")
    }

    pub fn set_password_hash(
        user_id: &i32,
        password_hash: &String,
        conn: &PgConnection,
    ) -> Result<usize, Error> {
        return diesel::update(
            users::dsl::users.filter(users::dsl::id.eq(user_id)),
        )
        .set(users::dsl::password_hash.eq(password_hash))
        .execute(conn);
    }
}
