use crate::error::AppError;
use chrono::{DateTime, Timelike, Utc};
use chrono_tz::{America, Tz};
use esp_idf_svc::sntp::{EspSntp, SyncStatus};
use std::time::SystemTime;

/// Type alias for the SNTP client using [EspSntp].
pub type Sntp = EspSntp<'static>;

/// Static constant representing the timezone.
pub static TIMEZONE: Tz = America::Sao_Paulo;

/// Initializes and returns an SNTP client with the default configuration.
///
/// This function creates and returns an instance of the [EspSntp] client, which is used
/// to synchronize the device's time with a network time server.
///
/// # Returns
/// - `Ok(EspSntp)`: The successfully created SNTP client instance.
/// - `Err(AppError)`: If there is an error during the SNTP client creation.
///
/// # Example
/// ```rust
/// let sntp = get_sntp().expect("Failed to initialize SNTP client");
/// ```
pub fn get_sntp() -> Result<EspSntp<'static>, AppError> {
    Ok(EspSntp::new_default()?)
}

/// Synchronizes the device's time with an SNTP server.
///
/// This function blocks the execution until the time synchronization is completed.
///
/// # Parameters
/// - `sntp`: A reference to the [Sntp] client that manages the synchronization process.
///
/// # Returns
/// `Ok(())` if the synchronization is successful, or an [AppError] if an error occurs.
///
/// # Example
/// ```rust
/// let sntp = get_sntp().expect("Failed to initialize SNTP client");
/// init_sntp(&sntp).expect("Failed to sync SNTP time");
/// ```
pub fn init_sntp(sntp: &Sntp) -> Result<(), AppError> {
    log::info!("Synchronizing with SNTP Server");
    while sntp.get_sync_status() != SyncStatus::Completed {}
    log::info!("Time Sync Completed");

    Ok(())
}

/// Retrieves the current time formatted as a vector of digits representing the hour and minute.
///
/// This function converts the current UTC time to the [TIMEZONE],
/// and then extracts the hour and minute components as a vector of 4 digits.
///
/// # Returns
/// A vector of 4 bytes representing the hour and minute, where each byte is a digit.
///
/// # Example
/// ```rust
/// let time = get_time();
/// ```
pub fn get_time() -> Vec<u8> {
    let now_utc: DateTime<Utc> = SystemTime::now().into();
    let now = now_utc.with_timezone(&TIMEZONE);
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
