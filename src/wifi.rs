use crate::error::AppError;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::modem::Modem,
    nvs::EspDefaultNvsPartition,
    wifi::{
        AuthMethod, BlockingWifi, ClientConfiguration, Configuration as WifiConfiguration, EspWifi,
    },
};
use std::str::FromStr;

/// Type alias for a [BlockingWifi] instance.
type Wifi = BlockingWifi<EspWifi<'static>>;

/// The SSID of the Wi-Fi network to connect to.
pub const WIFI_SSID: &str = "";

/// The password of the WiFi network.
const WIFI_PASSWORD: &str = "";

/// Creates a [BlockingWifi] instance for connecting to a Wi-Fi network.
///
/// This function creates and prepares the Wi-Fi instance.
/// The created instance can be used later for initializing and
/// connecting to a Wi-Fi network.
///
/// # Arguments
///
/// * `modem` - The modem instance used for networking.
///
/// # Returns
///
/// * `Ok(Wifi)` if the Wi-Fi instance is successfully created.
/// * `Err(AppError)` if any error occurs during the creation process.
///
/// # Example
/// ```rust
/// get_wifi(modem).expect("Error creating wifi");
/// ```
pub fn get_wifi(modem: Modem) -> Result<Wifi, AppError> {
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(EspWifi::new(modem, sysloop.clone(), Some(nvs))?, sysloop)?;

    wifi.set_configuration(&WifiConfiguration::Client(ClientConfiguration {
        ssid: heapless::String::from_str(WIFI_SSID).unwrap(),
        bssid: None,
        auth_method: AuthMethod::None,
        password: heapless::String::from_str(WIFI_PASSWORD).unwrap(),
        channel: None,
        ..Default::default()
    }))?;

    Ok(wifi)
}

/// Initializes and connects the Wi-Fi to the specified network.
///
/// This function starts the Wi-Fi, attempts to establish a connection to the
/// network using the provided configuration, and waits for the network interface
/// to be up. It checks the connection status repeatedly until the connection is
/// successfully established.
///
/// # Arguments
///
/// * `wifi` - A mutable reference to the [Wifi] instance.
///
/// # Returns
///
/// * `Ok(())` if the Wi-Fi connection is successfully established.
/// * `Err(AppError)` if any error occurs during the connection process.
///
/// # Example
/// ```rust
/// let mut wifi = get_wifi(modem).expect("Error creating wifi");
/// init_wifi(&mut wifi).expect("Error initializing wifi");
/// ```
pub fn init_wifi(wifi: &mut Wifi) -> Result<(), AppError> {
    wifi.start()?;
    wifi.connect()?;
    wifi.wait_netif_up()?;

    while !wifi.is_connected()? {
        let config = wifi.get_configuration()?;
        log::info!("Waiting for connection {:?}", config);
    }
    log::info!("Wifi Connected!");

    Ok(())
}
