pub const REGEX_RFC3339: &str =
    r"^([0-9]{4}-[0-9]{2}-[0-9]{2}[Tt][0-9]{2}:[0-9]{2}:[0-9]{2}[Zz+-:0-9]{1,6}$)";
pub const REGEX_RFC3339_NANO: &str =
    r"^([0-9]{4}-[0-9]{2}-[0-9]{2}[Tt][0-9]{2}:[0-9]{2}:[0-9]{2}.[0-9]{7,9}[Zz+-:0-9]{1,6}$)";
pub const REGEX_RFC1123: &str =
    r"^([A-Za-z]{3}, [0-9]{2} [A-Za-z]{3} [0-9]{4} [0-9]{2}:[0-9]{2}:[0-9]{2} [A-Za-z]{3,4}$)";
pub const REGEX_RFC1123Z: &str =
    r"^([A-Za-z]{3}, [0-9]{2} [A-Za-z]{3} [0-9]{4} [0-9]{2}:[0-9]{2}:[0-9]{2} [-+]{1}[0-9]{4}$)";
pub const REGEX_RFC822Z: &str =
    r"^([0-9]{2} [A-Za-z]{3} [0-9]{2} [0-9]{2}:[0-9]{2} [-+]{1}[0-9]{4}$)";
pub const REGEX_RFC822: &str = r"^([0-9]{2} [A-Za-z]{3} [0-9]{2} [0-9]{2}:[0-9]{2} [A-Za-z]{3,4}$)";
pub const REGEX_RFC850: &str =
    r"^([A-Za-z]{6,9}, [0-9]{2}-[A-Za-z]{3}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2} [A-Z]{3,4}$)";
pub const REGEX_RUBY_FORMAT: &str =
    r"^([A-Za-z]{3} [A-Za-z]{3} [0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2} [+-][0-9]{4} [0-9]{4}$)";
pub const REGEX_UNIX_FORMAT: &str =
    r"^([A-Za-z]{3} [A-Za-z]{3} [0-9_ ]{1,2} [0-9]{2}:[0-9]{2}:[0-9]{2} [A-Za-z]{3,4} [0-9]{4}$)";
pub const REGEX_ANSI_C_FORMAT: &str =
    r"^([A-Za-z]{3} [A-Za-z]{3} [0-9_ ]{1,2} [0-9]{2}:[0-9]{2}:[0-9]{2} [0-9]{4}$)";

pub const RFC1123: &str = "%a, %d %b %Y %H:%M:%S %Z";
pub const RFC1123Z: &str = "%a, %d %b %Y %H:%M:%S %z";
pub const RFC822: &str = "%d %b %y %H:%M %Z";
pub const RFC822Z: &str = "%d %b %y %H:%M %z";
pub const RFC3339_NANO: &str = "%Y-%m-%dT%H:%M:%S%.9f%:z";
pub const ANSI_C: &str = "%a %b %_d %H:%M:%S %Y";
pub const UNIX_FORMAT: &str = "%a %b %_d %H:%M:%S %Z %Y";
pub const RUBY_FORMAT: &str = "%a %b %_d %H:%M:%S %z %Y";
pub const RFC850: &str = "%A, %d-%b-%y %H:%M:%S %Z";
