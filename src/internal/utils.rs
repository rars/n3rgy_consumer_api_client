use chrono::{NaiveDateTime, ParseResult};

pub(crate) fn parse_n3rgy_timestamp(timestamp: &str) -> ParseResult<NaiveDateTime> {
    NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M")
}
