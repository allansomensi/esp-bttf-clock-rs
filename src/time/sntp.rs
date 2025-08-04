use crate::error::AppError;
use esp_idf_svc::sntp::{EspSntp, SyncStatus};

/// Initializes and returns an SNTP client with the default configuration.
///
/// This function creates and returns an instance of the [EspSntp] client, which
/// is used to synchronize the device's time with a network time server.
///
/// ## Returns
/// - `Ok(EspSntp)`: The successfully created SNTP client instance.
/// - `Err(AppError)`: If there is an error during the SNTP client creation.
///
/// ## Example
/// ```rust
/// let sntp = get_sntp().expect("Failed to initialize SNTP client");
/// ```
pub fn get_sntp() -> Result<EspSntp<'static>, AppError> {
    Ok(EspSntp::new_default()?)
}

/// Synchronizes the device's time with an SNTP server.
///
/// ## Arguments
/// - `sntp`: A reference to the [Sntp] client that manages the synchronization
///   process.
///
/// ## Returns
/// `Ok(())` if the synchronization is successful, or an [AppError] if an error
/// occurs.
///
/// ## Example
/// ```rust
/// let sntp = get_sntp().expect("Failed to initialize SNTP client");
/// init_sntp(&sntp).expect("Failed to sync SNTP time");
/// ```
pub fn init_sntp(sntp: &EspSntp<'static>) -> Result<(), AppError> {
    log::info!("Synchronizing with SNTP Server...");
    while sntp.get_sync_status() != SyncStatus::Completed {}
    log::info!("Time Sync Completed");

    Ok(())
}
