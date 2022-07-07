use chrono::{TimeZone, Utc};

/**
 * Convert time string to UNIX timestamp.
 * String format: "2012-06-28 12:29:00"
 */
pub fn convert_time_to_unix(time: String) -> u64 {
    let time = Utc.datetime_from_str(&time, "%Y-%m-%d %H:%M:%S").unwrap();
    let time = u64::try_from(time.timestamp()).unwrap();
    time
}

/**
 * Convert UNIX timestamp to time string.
 */
pub fn convert_unix_to_time(time: u64) -> String {
    let time = i64::try_from(time).unwrap();
    let time = Utc.timestamp(time, 0);
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub struct TimeConfig {
    pub time_start: Option<String>,
    pub time_end: Option<String>
}