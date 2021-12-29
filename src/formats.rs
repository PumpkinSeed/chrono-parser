pub const REGEX_RFC3339: &str =
    r"^([0-9]{4}-[0-9]{2}-[0-9]{2}[Tt][0-9]{2}:[0-9]{2}:[0-9]{2}[Zz+-:0-9]{1,6}$)";
pub const REGEX_RFC3339_NANO: &str =
    r"^([0-9]{4}-[0-9]{2}-[0-9]{2}[Tt][0-9]{2}:[0-9]{2}:[0-9]{2}.[0-9]{7,9}[Zz+-:0-9]{1,6}$)";
pub const REGEX_RFC1123: &str =
    r"^([A-Za-z]{3}, [0-9]{2} [A-Za-z]{3} [0-9]{4} [0-9]{2}:[0-9]{2}:[0-9]{2} [A-Za-z]{3,4}$)";
pub const REGEX_RFC1123Z: &str =
    r"^([A-Za-z]{3}, [0-9]{2} [A-Za-z]{3} [0-9]{4} [0-9]{2}:[0-9]{2}:[0-9]{2} [-+]{1}[0-9]{4}$)";
pub const REGEX_RFC822Z: &str = r"^([0-9]{2} [A-Za-z]{3} [0-9]{2} [0-9]{2}:[0-9]{2} [-+]{1}[0-9]{4}$)";
pub const REGEX_RFC822: &str = r"^([0-9]{2} [A-Za-z]{3} [0-9]{2} [0-9]{2}:[0-9]{2} [A-Za-z]{3,4}$)";
pub const REGEX_RFC850: &str =
    r"^([A-Za-z]{6,9}, [0-9]{2}-[A-Za-z]{3}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2} [A-Z]{3,4}$)";
pub const REGEX_RUBY_FORMAT: &str =
    r"^([A-Za-z]{3} [A-Za-z]{3} [0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2} [+-][0-9]{4} [0-9]{4}$)";
pub const REGEX_UNIX_FORMAT: &str =
    r"^([A-Za-z]{3} [A-Za-z]{3} [0-9_ ]{1,2} [0-9]{2}:[0-9]{2}:[0-9]{2} [A-Za-z]{3,4} [0-9]{4}$)";
pub const REGEX_ANSI_C_FORMAT: &str =
    r"^([A-Za-z]{3} [A-Za-z]{3} [0-9_ ]{1,2} [0-9]{2}:[0-9]{2}:[0-9]{2} [0-9]{4}$)";

// RFC1123     = "Mon, 02 Jan 2006 15:04:05 MST"
pub const RFC1123: &str = "%a, %d %b %Y %H:%M:%S %Z";

// RFC1123Z    = "Mon, 02 Jan 2006 15:04:05 -0700"
pub const RFC1123Z: &str = "%a, %d %b %Y %H:%M:%S %z";

// RFC822      = "02 Jan 06 15:04 MST"
pub const RFC822: &str = "%d %b %y %H:%M %Z";

// RFC822Z     = "02 Jan 06 15:04 -0700"
pub const RFC822Z: &str = "%d %b %y %H:%M %z";

// RFC3339Nano = "2006-01-02T15:04:05.999999999Z07:00"
pub const RFC3339_NANO: &str = "%Y-%m-%dT%H:%M:%S%.9f%:z";

// ANSIC       = "Mon Jan _2 15:04:05 2006"
pub const ANSI_C: &str = "%a %b %_d %H:%M:%S %Y";
// UnixDate    = "Mon Jan _2 15:04:05 MST 2006"
pub const UNIX_FORMAT: &str = "%a %b %_d %H:%M:%S %Z %Y";
// RubyDate    = "Mon Jan 02 15:04:05 -0700 2006"
pub const RUBY_FORMAT: &str = "%a %b %_d %H:%M:%S %z %Y";

// RFC850      = "Monday, 02-Jan-06 15:04:05 MST"
pub const RFC850: &str = "%A, %d-%b-%y %H:%M:%S %Z";
