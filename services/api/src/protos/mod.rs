pub mod time;

/// Include the `User` module, which is generated from user.proto.
pub mod user {
    include!(concat!(env!("OUT_DIR"), "/rnr.user.rs"));
}
