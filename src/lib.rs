use ::regex::Regex;
use chrono::Datelike;

pub mod formats;

pub fn parse(data: String) {
    let re = Regex::new(formats::RFC3339).unwrap();
    if re.is_match(data.as_str()) {
        let dt = chrono::DateTime::parse_from_rfc3339(data.as_str()).unwrap();
        println!("Test: {}", dt.year())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        crate::parse("2021-10-12T07:20:50.52Z".to_string())
    }
}
