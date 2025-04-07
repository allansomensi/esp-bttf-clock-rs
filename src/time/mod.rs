use chrono::{DateTime, Datelike, Timelike, Utc};
use std::{
    str::FromStr,
    time::{Duration, SystemTime},
};

pub mod sntp;
pub mod tz;

/// Retrieves the current time formatted as a vector of digits representing the
/// hour and minute.
///
/// This function converts the current UTC time to the [TIMEZONE],
/// and then extracts the hour and minute components as a vector of 4 digits.
///
/// ## Returns
/// A vector of 4 bytes representing the hour and minute, where each byte is a
/// digit.
///
/// ## Example
/// ```rust
/// let time = get_time();
/// ```
pub fn get_hour_min() -> Vec<u8> {
    let timezone = tz::get_timezone();
    let now_utc: DateTime<Utc> = SystemTime::now().into();
    let now =
        now_utc.with_timezone(&chrono_tz::Tz::from_str(&timezone).expect("Error reading Timezone"));
    let hour = now.hour();
    let minute = now.minute();

    let time_digits: [u8; 4] = [
        (hour / 10) as u8,
        (hour % 10) as u8,
        (minute / 10) as u8,
        (minute % 10) as u8,
    ];

    time_digits.into()
}

pub fn get_year() -> Vec<u8> {
    let timezone = tz::get_timezone();
    let now_utc: DateTime<Utc> = SystemTime::now().into();
    let now =
        now_utc.with_timezone(&chrono_tz::Tz::from_str(&timezone).expect("Error reading Timezone"));
    let year = now.year();

    let year_digits: [u8; 4] = [
        ((year / 1000) % 10) as u8,
        ((year / 100) % 10) as u8,
        ((year / 10) % 10) as u8,
        (year % 10) as u8,
    ];

    year_digits.into()
}

/// Calculates the time remaining until the next minute.
///
/// Returns a `Duration` representing the time to wait until the next exact
/// minute.
pub fn calculate_time_until_next_minute() -> Duration {
    let timezone = tz::get_timezone();
    let now_utc: DateTime<Utc> = SystemTime::now().into();
    let now_local =
        now_utc.with_timezone(&chrono_tz::Tz::from_str(&timezone).expect("Error reading Timezone"));

    let current_seconds = now_local.second();
    let seconds_to_wait = 60 - current_seconds;

    Duration::new(seconds_to_wait as u64, 0)
}
