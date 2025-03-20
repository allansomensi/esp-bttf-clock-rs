use super::get_wifi;
use crate::error::AppError;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{modem::WifiModemPeripheral, peripheral::Peripheral},
    nvs::EspDefaultNvsPartition,
    wifi::{
        AccessPointConfiguration, AuthMethod, BlockingWifi, Configuration as WifiConfiguration,
        EspWifi, WifiDriver,
    },
};

const AP_SSID: &str = "esp-clock";
const AP_PASS: &str = "bttf-rust";

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

fn configure_ap(wifi_ap: WifiDriver) -> Result<EspWifi, AppError> {
    let mut wifi_ap = EspWifi::wrap(wifi_ap)?;

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

pub fn connect_wifi_ap(wifi: &mut BlockingWifi<EspWifi<'static>>) -> Result<(), AppError> {
    wifi.start()?;
    log::info!("Wifi started!");

    wifi.wait_netif_up()?;
    log::info!("Wifi netif up!");

    Ok(())
}
