#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(message, optional, tag="2")]
    pub created_at: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub last_login: ::std::option::Option<::prost_types::Timestamp>,
    /// e.g. "17327706758"
    #[prost(string, tag="4")]
    pub phone: std::string::String,
    /// e.g. "install@pinksports.dev"
    #[prost(string, tag="5")]
    pub email: std::string::String,
    /// e.g. "boozinbuffalo"
    #[prost(string, tag="6")]
    pub username: std::string::String,
}
