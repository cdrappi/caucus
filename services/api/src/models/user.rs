use auth::jwt::{decode_jwt, JsonWebToken};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::PgConnection;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use schema::users;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub is_admin: bool,
    pub username: String,
    pub password_hash: String,
}

impl User {
    /// Can this user edit the votes of this org for this precinct?
    pub fn can_edit_votes(
        user_id: i32,
        org: &String,
        precinct: &String,
    ) -> bool {
        true
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.id)
    }

    pub fn get_from_id(conn: &PgConnection, id: i32) -> Result<User, Error> {
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
                User::create_from_username_password(conn, username, password)?;
                User::get_from_username(conn, username)
            }
        }
    }

    pub fn _set_username(
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

    pub fn set_admin(
        conn: &PgConnection,
        user_id: i32,
        is_admin: bool,
    ) -> Result<usize, Error> {
        return diesel::update(
            users::dsl::users.filter(users::dsl::id.eq(user_id)),
        )
        .set(users::dsl::is_admin.eq(is_admin))
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

    pub fn _set_password_hash(
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
