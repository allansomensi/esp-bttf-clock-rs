use crate::{
    error::AppError, prefs::hour_format::HourFormat, time::tz::TimezoneRequest,
    wifi::WifiCredentials,
};

/// Defines services for managing timezone settings in NVS.
pub trait AppStorageTzService {
    fn save_timezone(&mut self, timezone: TimezoneRequest) -> Result<(), AppError>;
    fn get_maybe_timezone(&mut self) -> Result<Option<String>, String>;
    fn delete_timezone(&mut self) -> Result<(), AppError>;
}

/// Defines services for managing Wi-Fi settings in NVS.
pub trait AppStorageWifiService {
    fn save_wifi_credentials(&mut self, ssid: String, password: String);
    fn get_maybe_wifi_credentials(&mut self) -> Result<Option<WifiCredentials>, String>;
    fn delete_wifi_credentials(&mut self) -> Result<(), AppError>;
}

/// Defines services for managing hour format in NVS.
pub trait AppStoragePrefsService {
    fn save_hour_format(&mut self, hour_format: HourFormat) -> Result<(), AppError>;
    fn get_maybe_hour_format(&mut self) -> Result<Option<HourFormat>, String>;
}
