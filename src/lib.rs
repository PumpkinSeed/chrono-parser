use ::regex::Regex;
use chrono::{DateTime, FixedOffset, ParseResult};

pub mod formats;

pub fn datetime(data: String) -> ParseResult<DateTime<FixedOffset>> {
    if Regex::new(formats::REGEX_RFC3339)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_rfc3339(data.as_str());
    }
    if Regex::new(formats::REGEX_RFC3339_NANO)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), formats::RFC3339_NANO);
    }
    if Regex::new(formats::REGEX_RFC1123)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), formats::RFC1123);
    }
    if Regex::new(formats::REGEX_RFC1123Z)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), formats::RFC1123Z);
    }
    if Regex::new(formats::REGEX_RFC822Z)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), formats::RFC822Z);
    }
    if Regex::new(formats::REGEX_RFC822)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), formats::RFC822);
    }
    if Regex::new(formats::REGEX_RFC850)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), formats::RFC850);
    }
    if Regex::new(formats::REGEX_RUBY_FORMAT)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), formats::RUBY_FORMAT);
    }
    if Regex::new(formats::REGEX_UNIX_FORMAT)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), formats::UNIX_FORMAT);
    }
    if Regex::new(formats::REGEX_ANSI_C_FORMAT)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), formats::ANSI_C);
    }

    return chrono::DateTime::parse_from_rfc2822(data.as_str());
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_parse() {
        match crate::datetime("2021-10-12T07:20:50.52Z".to_string()) {
            Ok(v) => {
                assert_eq!(v.year(), 2021);
                assert_eq!(v.month(), 10);
                assert_eq!(v.day(), 12);
                assert_eq!(v.hour(), 7);
                assert_eq!(v.minute(), 20);
                assert_eq!(v.second(), 50);
            }
            Err(err) => panic!("{}", err.to_string()),
        }
        match crate::datetime("02 Jan 06 15:04 -0700".to_string()) {
            Ok(v) => {
                assert_eq!(v.year(), 2006);
                assert_eq!(v.month(), 01);
                assert_eq!(v.day(), 02);
                assert_eq!(v.hour(), 15);
                assert_eq!(v.minute(), 4);
                assert_eq!(v.second(), 0);
            }
            Err(err) => panic!("{}", err.to_string()),
        }
        // Blocked by: https://github.com/chronotope/chrono/issues/623
        // Scenarios:
        // RFC1123 = "Mon, 02 Jan 2006 15:04:05 MST"
        // RFC1123Z = "Mon, 02 Jan 2006 15:04:05 -0700"
        // RFC822 = "02 Jan 06 15:04 MST"
        // RFC822Z = "02 Jan 06 15:04 -0700"
        // RFC3339Nano = "2006-01-02T15:04:05.999999999Z07:00"
        // ANSIC = "Mon Jan _2 15:04:05 2006"
        // UnixDate = "Mon Jan _2 15:04:05 MST 2006"
        // RubyDate = "Mon Jan 02 15:04:05 -0700 2006"
        // RFC850 = "Monday, 02-Jan-06 15:04:05 MST"
    }
}
