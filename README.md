# chrono parser

Date, time parser in any format. Recognize the format and parse to `DateTime<FixedOffset>`.

Known formats:

- RFC1123
- RFC1123Z
- RFC822
- RFC822Z
- RFC3339_NANO
- ANSI_C
- UNIX_FORMAT
- RUBY_FORMAT
- RFC850

### Usage

```rust
use chrono_parser::datetime;

fn main() {
    match datetime("02 Jan 06 15:04 -0700".to_string()) {
        Ok(v) => {...}
        Err(err) => panic!("{}", err.to_string()),
    }
}
```
