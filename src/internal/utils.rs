use chrono::{NaiveDateTime, ParseResult};

pub(crate) fn parse_n3rgy_timestamp(timestamp: &str) -> ParseResult<NaiveDateTime> {
    NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate, Timelike};

    #[test]
    fn test_parse_valid_timestamp() {
        let result = parse_n3rgy_timestamp("2023-09-17 14:30");
        assert!(result.is_ok());
        let datetime = result.unwrap();
        assert_eq!(datetime.year(), 2023);
        assert_eq!(datetime.month(), 9);
        assert_eq!(datetime.day(), 17);
        assert_eq!(datetime.hour(), 14);
        assert_eq!(datetime.minute(), 30);
    }

    #[test]
    fn test_parse_invalid_format() {
        let result = parse_n3rgy_timestamp("2023/09/17 14:30");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_date() {
        let result = parse_n3rgy_timestamp("2023-13-32 14:30");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_time() {
        let result = parse_n3rgy_timestamp("2023-09-17 25:61");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_string() {
        let result = parse_n3rgy_timestamp("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_partial_timestamp() {
        let result = parse_n3rgy_timestamp("2023-09-17");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_leap_year() {
        let result = parse_n3rgy_timestamp("2024-02-29 00:00");
        assert!(result.is_ok());
        let datetime = result.unwrap();
        assert_eq!(
            datetime.date(),
            NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
        );
    }

    #[test]
    fn test_parse_non_leap_year() {
        let result = parse_n3rgy_timestamp("2023-02-29 00:00");
        assert!(result.is_err());
    }
}
