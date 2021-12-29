use ::regex::Regex;
use chrono::Datelike;

pub mod formats;

pub fn parse(data: Box<dyn std::string::ToString>) {
    let d = data.to_string();

    let re = Regex::new(formats::RFC3339).unwrap();
    if re.is_match(d.as_str()) {
        let dt = chrono::DateTime::parse_from_rfc3339(d.as_str()).unwrap();
        println!("Test: {}", dt.year())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        crate::parse(Box::new("2021-10-12T07:20:50.52Z"))
    }
}
