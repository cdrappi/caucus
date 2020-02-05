use diesel::PgConnection;

#[database("postgres_database")]
pub struct DbConn(PgConnection);
