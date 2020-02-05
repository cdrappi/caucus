use std::{env, time::SystemTime};

use bytes::Bytes;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Body, Responder, Response};
use rocket_contrib::json::JsonValue;
use std::io::Cursor;

pub fn expect_env(name: &str) -> String {
    return env::var(name).expect(&format!("Missing `{}` env variable", name));
}

pub fn now_seconds() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("SystemTime before UNIX Epoch!")
        .as_secs()
}

pub fn to_string(value: &Option<String>) -> String {
    value.as_ref().unwrap_or(&"".to_string()).to_string()
}

#[derive(Debug)]
pub struct JsonResponse {
    pub status: Status,
    pub json: JsonValue,
}

impl JsonResponse {
    pub fn ok(json: JsonValue) -> Self {
        JsonResponse {
            status: Status::Ok,
            json: json,
        }
    }

    pub fn err400(json: JsonValue) -> Self {
        JsonResponse {
            status: Status::BadRequest,
            json: json,
        }
    }

    pub fn err500(json: JsonValue) -> Self {
        JsonResponse {
            status: Status::InternalServerError,
            json: json,
        }
    }
}

impl<'r> response::Responder<'r> for JsonResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        response::Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[derive(Debug)]
pub struct ProtobufResponse {
    pub status: Status,
    pub body: Bytes,
}

impl ProtobufResponse {
    pub fn ok(body: Bytes) -> Self {
        ProtobufResponse {
            status: Status::Ok,
            body: body,
        }
    }

    pub fn _err(body: Bytes) -> Self {
        ProtobufResponse {
            status: Status::Unauthorized,
            body: body,
        }
    }
}

impl<'r> Responder<'r> for ProtobufResponse {
    fn respond_to(self, _req: &Request) -> response::Result<'r> {
        Response::build()
            .status(self.status)
            .header(ContentType::new("application", "protobuf"))
            .raw_body(Body::Sized(Cursor::new(self.body), 8096))
            .ok()
    }
}
