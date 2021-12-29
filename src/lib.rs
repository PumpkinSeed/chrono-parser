use ::regex::Regex;
use chrono::Datelike;
use chrono::{DateTime, FixedOffset, ParseResult};

pub mod formats;

pub fn datetime(data: String) -> ParseResult<DateTime<FixedOffset>> {
    if Regex::new(formats::RFC3339)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_rfc3339(data.as_str());
    }
    if Regex::new(formats::RFC3339_NANO)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), "");
    }
    if Regex::new(formats::RFC1123)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), "");
    }
    if Regex::new(formats::RFC1123Z)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), "");
    }
    if Regex::new(formats::RFC822Z)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), "");
    }
    if Regex::new(formats::RFC822).unwrap().is_match(data.as_str()) {
        return chrono::DateTime::parse_from_str(data.as_str(), "");
    }
    if Regex::new(formats::RFC850).unwrap().is_match(data.as_str()) {
        return chrono::DateTime::parse_from_str(data.as_str(), "");
    }
    if Regex::new(formats::RUBY_FORMAT)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), "");
    }
    if Regex::new(formats::UNIX_FORMAT)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), "");
    }
    if Regex::new(formats::ANSIC_FORMAT)
        .unwrap()
        .is_match(data.as_str())
    {
        return chrono::DateTime::parse_from_str(data.as_str(), "");
    }

    return chrono::DateTime::parse_from_rfc2822(data.as_str());
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;

    #[test]
    fn test_parse() {
        match crate::datetime("2021-10-12T07:20:50.52Z".to_string()) {
            Ok(v) => {
                println!("{}", v.year());
            }
            Err(err) => panic!("{}", err.to_string()),
        }
    }
}
