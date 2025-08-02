use super::AppStorage;
use crate::{error::AppError, service::app_storage::AppStorageWifiService, wifi::WifiCredentials};
use postcard::{from_bytes, to_vec};

pub const WIFI_NAMESPACE: &str = "wifi_ns";

impl AppStorageWifiService for AppStorage {
    /// Saves Wi-Fi credentials to NVS storage.
    ///
    /// ## Arguments
    ///
    /// * `wifi_nvs` - A mutable reference to the ESP NVS storage.
    /// * `ssid` - The Wi-Fi SSID as a `String`.
    /// * `password` - The Wi-Fi password as a `String`.
    ///
    /// ## Behavior
    ///
    /// Stores the provided SSID and password under the key `"net_info"`.
    /// If the operation succeeds, logs a success message; otherwise, logs an
    /// error message.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let mut wifi_nvs = initialize_nvs(); // Assume this function initializes NVS.
    /// save_wifi_credentials(
    ///     &mut wifi_nvs,
    ///     "MyNetwork".to_string(),
    ///     "SecurePass123".to_string(),
    /// );
    /// ```
    fn save_wifi_credentials(&mut self, ssid: String, password: String) {
        let key_wifi_credentials: &str = "net_info";
        let key_wifi_credentials_data = WifiCredentials { ssid, password };

        match self.wifi_nvs.set_raw(
            key_wifi_credentials,
            &to_vec::<WifiCredentials, 100>(&key_wifi_credentials_data).unwrap(),
        ) {
            Ok(_) => log::info!("Key {key_wifi_credentials} updated"),
            Err(e) => log::error!("key {key_wifi_credentials} not updated {e:?}"),
        };
    }

    /// Retrieves stored Wi-Fi credentials from NVS, if available.
    ///
    /// ## Arguments
    ///
    /// * `wifi_nvs` - A mutable reference to the ESP NVS storage.
    ///
    /// ## Returns
    ///
    /// * `Ok(Some(WifiCredentials))` - If credentials are found and
    ///   successfully deserialized.
    /// * `Ok(None)` - If no credentials are stored.
    /// * `Err(String)` - If an error occurs during retrieval or
    ///   deserialization.
    ///
    /// ## Behavior
    ///
    /// Attempts to fetch and deserialize Wi-Fi credentials from the
    /// `"net_info"` key. If retrieval or deserialization fails, returns an
    /// error message.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let mut wifi_nvs = initialize_nvs(); // Assume this function initializes NVS.
    /// match get_maybe_wifi_credentials(&mut wifi_nvs) {
    ///     Ok(Some(credentials)) => println!(
    ///         "SSID: {}, Password: {}",
    ///         credentials.ssid, credentials.password
    ///     ),
    ///     Ok(None) => println!("No credentials found."),
    ///     Err(e) => eprintln!("Error retrieving credentials: {e}"),
    /// }
    /// ```
    fn get_maybe_wifi_credentials(&mut self) -> Result<Option<WifiCredentials>, String> {
        let key_wifi_credentials = "net_info";
        let mut key_wifi_credentials_data = [0u8; 100];

        match self
            .wifi_nvs
            .get_raw(key_wifi_credentials, &mut key_wifi_credentials_data)
        {
            Ok(Some(credentials_bytes)) => from_bytes::<WifiCredentials>(credentials_bytes)
                .map(Some)
                .map_err(|e| format!("Failed to deserialize Wi-Fi credentials: {e:?}")),
            Ok(None) => Ok(None),
            Err(e) => Err(format!(
                "Couldn't get key {key_wifi_credentials} because {e:?}"
            )),
        }
    }

    /// Deletes stored Wi-Fi credentials from NVS.
    ///
    /// ## Arguments
    ///
    /// * `wifi_nvs` - A mutable reference to the ESP NVS storage.
    ///
    /// ## Behavior
    ///
    /// Removes the Wi-Fi credentials stored under the `"net_info"` key.
    /// If the operation succeeds, logs a success message; otherwise, logs an
    /// error message.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let mut wifi_nvs = initialize_nvs(); // Assume this function initializes NVS.
    /// delete_wifi_credentials(&mut wifi_nvs);
    /// ```
    fn delete_wifi_credentials(&mut self) -> Result<(), AppError> {
        let key_wifi_credentials: &str = "net_info";

        match self.wifi_nvs.remove(key_wifi_credentials) {
            Ok(_) => log::info!("Key {key_wifi_credentials} deleted"),
            Err(e) => log::error!("key {key_wifi_credentials} not deleted {e:?}"),
        };

        Ok(())
    }
}
