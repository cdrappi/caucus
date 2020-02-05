use chrono::NaiveDateTime;
use prost_types::Timestamp;

pub fn chrono_to_prost(dt: &NaiveDateTime) -> Option<Timestamp> {
    Some(Timestamp {
        seconds: dt.timestamp(),
        nanos: 0,
    })
}
