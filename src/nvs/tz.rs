use super::AppStorage;
use crate::{
    error::AppError, service::app_storage::AppStorageTzService, time::tz::TimezoneRequest,
};

pub const TZ_NAMESPACE: &str = "tz_ns";

impl AppStorageTzService for AppStorage {
    /// Saves the provided timezone information to NVS storage.
    ///
    /// ## Arguments
    ///
    /// * `tz_nvs` - A mutable reference to the ESP NVS storage.
    /// * `timezone` - A [TimezoneRequest] containing the timezone string.
    ///
    /// ## Behavior
    ///
    /// Stores the provided timezone under the key `"tz_info"`. If the operation
    /// succeeds, logs a success message; otherwise, logs an error message.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let mut tz_nvs = initialize_nvs(); // Assume this function initializes NVS.
    /// save_timezone(
    ///     &mut tz_nvs,
    ///     TimezoneRequest {
    ///         timezone: "UTC".to_string(),
    ///     },
    /// );
    /// ```
    fn save_timezone(&mut self, timezone: TimezoneRequest) -> Result<(), AppError> {
        let key_timezone: &str = "tz_info";
        let key_timezone_data = &timezone.timezone;

        match self.tz_nvs.set_str(key_timezone, key_timezone_data) {
            Ok(_) => log::info!("Key {key_timezone} updated"),
            Err(e) => log::error!("key {key_timezone} not updated {:?}", e),
        };

        Ok(())
    }

    /// Retrieves the stored timezone from NVS, if available.
    ///
    /// ## Arguments
    ///
    /// * `tz_nvs` - A mutable reference to the ESP NVS storage.
    ///
    /// ## Returns
    ///
    /// * `Ok(Some(String))` - If a timezone is found and successfully
    ///   retrieved.
    /// * `Ok(None)` - If no timezone is stored.
    /// * `Err(String)` - If an error occurs during retrieval.
    ///
    /// ## Behavior
    ///
    /// Attempts to fetch the stored timezone from the `"tz_info"` key. If
    /// retrieval fails, returns an error message.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let mut tz_nvs = initialize_nvs(); // Assume this function initializes NVS.
    /// match get_maybe_timezone(&mut tz_nvs) {
    ///     Ok(Some(timezone)) => println!("Stored timezone: {}", timezone),
    ///     Ok(None) => println!("No timezone found."),
    ///     Err(e) => eprintln!("Error retrieving timezone: {}", e),
    /// }
    /// ```
    fn get_maybe_timezone(&mut self) -> Result<Option<String>, String> {
        let key_timezone = "tz_info";
        let mut key_timezone_data = [0u8; 100];

        match self.tz_nvs.get_str(key_timezone, &mut key_timezone_data) {
            Ok(Some(timezone_str)) => Ok(Some(timezone_str.to_string())),
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Couldn't get key {key_timezone} because {:?}", e)),
        }
    }

    /// Deletes the stored timezone information from NVS.
    ///
    /// ## Arguments
    ///
    /// * `tz_nvs` - A mutable reference to the ESP NVS storage.
    ///
    /// ## Behavior
    ///
    /// Removes the stored timezone information under the `"tz_info"` key.
    /// If the operation succeeds, logs a success message; otherwise, logs an
    /// error message.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let mut tz_nvs = initialize_nvs(); // Assume this function initializes NVS.
    /// delete_timezone(&mut tz_nvs);
    /// ```
    fn delete_timezone(&mut self) -> Result<(), AppError> {
        let key_timezone: &str = "tz_info";

        match self.tz_nvs.remove(key_timezone) {
            Ok(_) => log::info!("Key {key_timezone} deleted"),
            Err(e) => log::error!("key {key_timezone} not deleted {:?}", e),
        };

        Ok(())
    }
}
