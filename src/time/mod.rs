use chrono::{DateTime, Timelike, Utc};
use chrono_tz::Tz;
use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

pub mod sntp;

lazy_static::lazy_static! {
    /// Global static reference for storing the Timezone.
    ///
    /// This global reference uses `lazy_static` to initialize a `Arc<Mutex>` that holds an
    /// `Option<Tz>`. It can be used across the application to store and retrieve
    /// the Timezone in a thread-safe manner.
    pub static ref TIMEZONE: Arc<Mutex<Option<Tz>>> = Arc::new(Mutex::new(Some(Tz::UTC)));
}

/// Retrieves the current time formatted as a vector of digits representing the hour and minute.
///
/// This function converts the current UTC time to the [TIMEZONE],
/// and then extracts the hour and minute components as a vector of 4 digits.
///
/// ## Returns
/// A vector of 4 bytes representing the hour and minute, where each byte is a digit.
///
/// ## Example
/// ```rust
/// let time = get_time();
/// ```
pub fn get_time() -> Vec<u8> {
    let now_utc: DateTime<Utc> = SystemTime::now().into();
    let now = now_utc.with_timezone(&TIMEZONE.lock().unwrap().unwrap());
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
