use super::AppStorage;
use crate::{
    error::AppError, prefs::hour_format::HourFormat, service::app_storage::AppStoragePrefsService,
};

/// The namespace used in NVS to store all user preferences.
pub const PREFS_NAMESPACE: &str = "prefs_ns";

impl AppStoragePrefsService for AppStorage {
    /// Saves the user's selected hour format setting to NVS.
    fn save_hour_format(&mut self, hour_format: HourFormat) -> Result<(), AppError> {
        let key_hour_format: &str = "hour_format";
        let hour_format_data: u8 = hour_format as u8;

        match self.prefs_nvs.set_u8(key_hour_format, hour_format_data) {
            Ok(_) => log::info!("Key '{key_hour_format}' updated in NVS."),
            Err(e) => log::error!("Key '{key_hour_format}' could not be updated in NVS: {e:?}",),
        };

        Ok(())
    }

    /// Retrieves the hour format setting from NVS.
    fn get_maybe_hour_format(&mut self) -> Result<Option<HourFormat>, String> {
        let key_hour_format = "hour_format";

        match self.prefs_nvs.get_u8(key_hour_format) {
            Ok(Some(hour_format_value)) => Ok(Some(HourFormat::from(hour_format_value))),
            Ok(None) => Ok(None),
            Err(e) => Err(format!(
                "Couldn't get key '{key_hour_format}' because: {e:?}",
            )),
        }
    }
}
