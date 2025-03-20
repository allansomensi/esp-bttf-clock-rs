use super::get_wifi;
use crate::error::AppError;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{modem::WifiModemPeripheral, peripheral::Peripheral},
    nvs::EspDefaultNvsPartition,
    wifi::{
        AuthMethod, BlockingWifi, ClientConfiguration, Configuration as WifiConfiguration, EspWifi,
        WifiDriver,
    },
};

/// Initializes the Wi-Fi station and connects to the specified network.
///
/// This function sets up the Wi-Fi driver in station mode, configures it with the provided SSID
/// and password, and starts the Wi-Fi connection process. It returns a [BlockingWifi] instance wrapped
/// in [EspWifi], which can be used for blocking Wi-Fi operations.
///
/// ## Arguments
/// - `modem`: The Wi-Fi modem peripheral to use.
/// - `sysloop`: The system event loop for managing events.
/// - `nvs`: Optional NVS partition for storing Wi-Fi credentials.
/// - `ssid`: The SSID of the Wi-Fi network to connect to.
/// - `password`: The password for the Wi-Fi network.
///
/// ## Returns
/// - `Result<BlockingWifi<EspWifi<'d>>, AppError>`: A wrapped [BlockingWifi] instance on success or an error.
///
/// ## Example
/// ```rust
/// let ssid = "MyNetwork".to_string();
/// let password = "MyPassword".to_string();
/// let wifi = get_station(modem, sysloop, nvs, ssid, password);
/// match wifi {
///     Ok(wifi) => println!("Wi-Fi connected successfully!"),
///     Err(e) => eprintln!("Failed to connect to Wi-Fi: {:?}", e),
/// }
/// ```
pub fn get_station<'d, M>(
    modem: impl Peripheral<P = M> + 'd,
    sysloop: EspSystemEventLoop,
    nvs: Option<EspDefaultNvsPartition>,
    ssid: String,
    password: String,
) -> Result<BlockingWifi<EspWifi<'d>>, AppError>
where
    M: WifiModemPeripheral,
{
    let wifi = get_wifi(modem, sysloop.clone(), nvs)?;
    let wifi = configure_station(wifi, ssid, password)?;
    let wifi = BlockingWifi::wrap(wifi, sysloop)?;

    Ok(wifi)
}

/// Configures the Wi-Fi driver for station mode with the specified SSID and password.
///
/// This function sets up the Wi-Fi configuration for connecting to a Wi-Fi network in client mode.
/// It uses the WPA2 Personal authentication method and sets the provided SSID and password.
///
/// # Arguments
/// - `wifi`: The `WifiDriver` instance to configure.
/// - `ssid`: The SSID of the Wi-Fi network.
/// - `password`: The password for the Wi-Fi network.
///
/// # Returns
/// - `Result<EspWifi, AppError>`: The configured `EspWifi` instance on success or an error.
///
/// # Example
/// ```rust
/// let ssid = "MyNetwork".to_string();
/// let password = "MyPassword".to_string();
/// let wifi_driver = get_wifi_driver();  // Hypothetical function to get the WifiDriver instance
/// match configure_station(wifi_driver, ssid, password) {
///     Ok(wifi) => println!("Wi-Fi configured successfully!"),
///     Err(e) => eprintln!("Failed to configure Wi-Fi: {:?}", e),
/// }
/// ```
fn configure_station(
    wifi: WifiDriver,
    ssid: String,
    password: String,
) -> Result<EspWifi, AppError> {
    let mut wifi = EspWifi::wrap(wifi)?;

    let wifi_configuration = WifiConfiguration::Client(ClientConfiguration {
        ssid: ssid.as_str().try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: password.as_str().try_into().unwrap(),
        channel: None,
        ..Default::default()
    });
    wifi.set_configuration(&wifi_configuration)?;

    Ok(wifi)
}

/// Starts the Wi-Fi connection process and waits until the device is connected.
///
/// This function starts the Wi-Fi connection process, waits for the network interface to be up,
/// and ensures the device is successfully connected to the network.
///
/// ## Arguments
/// - `wifi`: A mutable reference to the [BlockingWifi] instance wrapped with [EspWifi]
///
/// ## Returns
/// - `Result<(), AppError>`: A result indicating success or an error.
///
/// ## Example
/// ```rust
/// let mut wifi = get_station(modem, sysloop, nvs, ssid, password).unwrap();
/// match connect_wifi(&mut wifi) {
///     Ok(()) => println!("Wi-Fi connected successfully!"),
///     Err(e) => eprintln!("Failed to connect to Wi-Fi: {:?}", e),
/// }
/// ```
pub fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> Result<(), AppError> {
    wifi.start()?;
    log::info!("Wifi started!");

    wifi.connect()?;
    log::info!("Wifi connected!");

    wifi.wait_netif_up()?;
    log::info!("Wifi netif up!");

    while !wifi.is_connected()? {
        let config = wifi.get_configuration()?;
        log::info!("Waiting for connection... {:?}", config);
    }
    log::info!("Wifi done!");

    Ok(())
}
