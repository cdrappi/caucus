table! {
    users (id) {
        id -> Int4,
        created_at -> Timestamp,
        last_login -> Timestamp,
        password_hash -> Varchar,
        phone -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
    }
}
