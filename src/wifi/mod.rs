use std::sync::{Arc, Mutex};

use crate::error::AppError;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{modem::WifiModemPeripheral, peripheral::Peripheral},
    nvs::EspDefaultNvsPartition,
    wifi::WifiDriver,
};
use serde::{Deserialize, Serialize};

pub mod ap;
pub mod station;

lazy_static::lazy_static! {
    pub static ref WIFI_CREDENTIALS: Arc<Mutex<Option<WifiCredentials>>> = Arc::new(Mutex::new(None));
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiCredentials {
    pub ssid: String,
    pub password: String,
}

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
