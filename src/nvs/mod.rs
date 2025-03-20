use crate::wifi::WifiCredentials;
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use postcard::{from_bytes, to_vec};

#[allow(unused)]
pub fn save_wifi_credentials(nvs: &mut EspNvs<NvsDefault>, ssid: String, password: String) {
    let key_wifi_credentials: &str = "net_info";
    let key_wifi_credentials_data = WifiCredentials { ssid, password };

    match nvs.set_raw(
        key_wifi_credentials,
        &to_vec::<WifiCredentials, 100>(&key_wifi_credentials_data).unwrap(),
    ) {
        Ok(_) => log::info!("Key {key_wifi_credentials} updated"),
        Err(e) => log::error!("key {key_wifi_credentials} not updated {:?}", e),
    };
}

pub fn get_maybe_wifi_credentials(
    nvs: &mut EspNvs<NvsDefault>,
) -> Result<Option<WifiCredentials>, String> {
    let key_wifi_credentials = "net_info";
    let mut key_wifi_credentials_data = [0u8; 100];

    match nvs.get_raw(key_wifi_credentials, &mut key_wifi_credentials_data) {
        Ok(Some(credentials_bytes)) => from_bytes::<WifiCredentials>(credentials_bytes)
            .map(Some)
            .map_err(|e| format!("Failed to deserialize Wi-Fi credentials: {:?}", e)),
        Ok(None) => Ok(None),
        Err(e) => Err(format!(
            "Couldn't get key {key_wifi_credentials} because {:?}",
            e
        )),
    }
}

#[allow(unused)]
pub fn delete_wifi_credentials(nvs: &mut EspNvs<NvsDefault>) {
    let key_wifi_credentials: &str = "net_info";

    match nvs.remove(key_wifi_credentials) {
        Ok(_) => log::info!("Key {key_wifi_credentials} deleted"),
        Err(e) => log::error!("key {key_wifi_credentials} not deleted {:?}", e),
    };
}
