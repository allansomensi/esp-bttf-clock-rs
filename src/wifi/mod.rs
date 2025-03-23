use crate::error::AppError;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{modem::WifiModemPeripheral, peripheral::Peripheral},
    nvs::EspDefaultNvsPartition,
    wifi::WifiDriver,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub mod ap;
pub mod station;

lazy_static::lazy_static! {
    /// Global static reference for storing Wi-Fi credentials.
    ///
    /// This global reference uses `lazy_static` to initialize a `Arc<Mutex>` that holds an
    /// `Option<WifiCredentials>`. It can be used across the application to store and retrieve
    /// the Wi-Fi credentials in a thread-safe manner.
    pub static ref WIFI_CREDENTIALS: Arc<Mutex<Option<WifiCredentials>>> = Arc::new(Mutex::new(None));
}

/// Represents Wi-Fi credentials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiCredentials {
    pub ssid: String,
    pub password: String,
}

/// Initializes a [WifiDriver] instance with the provided modem, event loop, and
/// optional NVS partition.
///
/// This function creates a new [WifiDriver] instance using the provided
/// peripheral modem, system event loop, and optional NVS partition for storing
/// settings. It is used to set up Wi-Fi functionality on the device.
///
/// ## Arguments
/// - `modem`: The Wi-Fi modem peripheral to use.
/// - `sysloop`: The system event loop for managing events related to Wi-Fi.
/// - `nvs`: Optional NVS partition for storing Wi-Fi credentials and other
///   settings.
///
/// ## Returns
/// - `Result<WifiDriver<'d>, AppError>`: Returns the configured [WifiDriver]
///   instance on success or an error.
///
/// ## Example
/// ```rust
/// let wifi_driver = get_wifi(modem, sysloop, nvs);
/// match wifi_driver {
///     Ok(driver) => println!("Wi-Fi driver initialized successfully!"),
///     Err(e) => eprintln!("Error initializing Wi-Fi driver: {:?}", e),
/// }
/// ```
pub fn get_wifi<'d, M>(
    modem: impl Peripheral<P = M> + 'd,
    sysloop: EspSystemEventLoop,
    nvs: Option<EspDefaultNvsPartition>,
) -> Result<WifiDriver<'d>, AppError>
where
    M: WifiModemPeripheral,
{
    let wifi = WifiDriver::new(modem, sysloop, nvs)?;

    Ok(wifi)
}
