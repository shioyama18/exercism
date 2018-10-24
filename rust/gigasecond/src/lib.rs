extern crate chrono;
use chrono::{DateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + chrono::Duration::seconds(1_000_000_000)
}
