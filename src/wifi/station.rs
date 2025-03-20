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
