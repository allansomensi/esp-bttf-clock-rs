use std::{net::Ipv4Addr, str::FromStr};

use super::get_wifi;
use crate::error::AppError;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{modem::WifiModemPeripheral, peripheral::Peripheral},
    ipv4::{self, Mask, RouterConfiguration, Subnet},
    netif::{EspNetif, NetifConfiguration, NetifStack},
    nvs::EspDefaultNvsPartition,
    wifi::{
        AccessPointConfiguration, AuthMethod, BlockingWifi, Configuration as WifiConfiguration,
        EspWifi, WifiDriver,
    },
};

pub const AP_IP_ADDRESS: &str = env!("AP_IP_ADDRESS");
const AP_SSID: &str = "esp-clock";
const AP_PASS: &str = "bttf-rust";

/// Creates and configures an Access Point (AP) mode Wi-Fi instance.
///
/// ## Arguments
///
/// - `modem`: The Wi-Fi modem peripheral.
/// - `sysloop`: The system event loop for handling Wi-Fi events.
/// - `nvs`: Optional Non-Volatile Storage partition for saving Wi-Fi settings.
///
/// ## Returns
///
/// - `Ok(BlockingWifi<EspWifi>)`: A blocking Wi-Fi instance configured as an Access Point.
/// - `Err(AppError)`: If there is a failure in setting up the AP.
///
/// ## Example
///
/// ```rust
/// let wifi_ap = get_ap(modem, sysloop, nvs)?;
/// ```
pub fn get_ap<'d, M>(
    modem: impl Peripheral<P = M> + 'd,
    sysloop: EspSystemEventLoop,
    nvs: Option<EspDefaultNvsPartition>,
) -> Result<BlockingWifi<EspWifi<'d>>, AppError>
where
    M: WifiModemPeripheral,
{
    let wifi = get_wifi(modem, sysloop.clone(), nvs)?;
    let wifi_ap = configure_ap(wifi)?;
    let wifi_ap = BlockingWifi::wrap(wifi_ap, sysloop)?;

    Ok(wifi_ap)
}

/// Configures the Wi-Fi module as an Access Point (AP) with predefined settings.
///
/// This function sets up the Wi-Fi module in Access Point mode with the following configuration:
/// - SSID: [AP_SSID]
/// - Password: [AP_PASS]
/// - Authentication: WPA2-Personal
/// - Maximum number of connections: 4
/// - IP configuration: [AP_IP_ADDRESS]
///
/// ## Arguments
///
/// - `wifi_ap`: The Wi-Fi driver instance.
///
/// ## Returns
///
/// - `Ok(EspWifi)`: The configured Wi-Fi instance.
/// - `Err(AppError)`: If an error occurs during configuration.
///
/// ## Example
///
/// ```rust
/// let wifi_ap = configure_ap(wifi_driver)?;
/// ```
fn configure_ap(wifi_ap: WifiDriver) -> Result<EspWifi, AppError> {
    let ap_ip_address = Ipv4Addr::from_str(AP_IP_ADDRESS).expect("Error reading AP_IP_ADDRESS");

    let mut wifi_ap = EspWifi::wrap_all(
        wifi_ap,
        EspNetif::new(NetifStack::Sta)?,
        EspNetif::new_with_conf(&NetifConfiguration {
            ip_configuration: Some(ipv4::Configuration::Router(RouterConfiguration {
                subnet: Subnet {
                    gateway: ap_ip_address,
                    mask: Mask(24),
                },
                dhcp_enabled: true,
                dns: Some(ap_ip_address),
                secondary_dns: Some(ap_ip_address),
            })),
            ..NetifConfiguration::wifi_default_router()
        })?,
    )?;

    let wifi_configuration = WifiConfiguration::AccessPoint(AccessPointConfiguration {
        ssid: AP_SSID.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        password: AP_PASS.try_into().unwrap(),
        max_connections: 4,
        ..Default::default()
    });
    wifi_ap.set_configuration(&wifi_configuration)?;

    Ok(wifi_ap)
}

/// Starts the Wi-Fi Access Point and waits until the network interface is up.
///
/// ## Parameters
///
/// - `wifi`: A mutable reference to a [BlockingWifi] instance.
///
/// ## Returns
///
/// - `Ok(())`: If the AP starts successfully.
/// - `Err(AppError)`: If the AP fails to start.
///
/// ## Example
///
/// ```rust
/// start_wifi_ap(&mut wifi_ap)?;
/// ```
pub fn start_wifi_ap(wifi: &mut BlockingWifi<EspWifi<'static>>) -> Result<(), AppError> {
    wifi.start()?;
    log::info!("Wifi started!");

    wifi.wait_netif_up()?;
    log::info!("Wifi netif up!");

    Ok(())
}
