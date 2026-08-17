use time::{PrimitiveDateTime as DateTime, PlainDateTime};
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const GIGASECONDS: i64 = 1_000_000_000;
    return start +  Duration::seconds(GIGASECONDS);
}
