use crate::{error::AppError, time::tz::TimezoneRequest, wifi::WifiCredentials};

pub trait AppStorageTzService {
    fn save_timezone(&mut self, timezone: TimezoneRequest) -> Result<(), AppError>;
    fn get_maybe_timezone(&mut self) -> Result<Option<String>, String>;
    fn delete_timezone(&mut self) -> Result<(), AppError>;
}

pub trait AppStorageWifiService {
    fn save_wifi_credentials(&mut self, ssid: String, password: String);
    fn get_maybe_wifi_credentials(&mut self) -> Result<Option<WifiCredentials>, String>;
    fn delete_wifi_credentials(&mut self) -> Result<(), AppError>;
}
