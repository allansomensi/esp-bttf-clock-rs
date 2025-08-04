use crate::error::AppError;
use esp_idf_svc::nvs::{EspNvs, EspNvsPartition, NvsDefault};
use std::sync::{Arc, Mutex};
use tz::TZ_NAMESPACE;
use wifi::WIFI_NAMESPACE;

pub mod tz;
pub mod wifi;

pub type SharedAppStorage = Arc<Mutex<AppStorage>>;

/// Serves as a centralized container for managing different types of data
/// stored in NVS.
pub struct AppStorage {
    pub wifi_nvs: EspNvs<NvsDefault>,
    pub tz_nvs: EspNvs<NvsDefault>,
}

impl AppStorage {
    pub fn new(
        nvs_default_partition: EspNvsPartition<NvsDefault>,
    ) -> Result<SharedAppStorage, AppError> {
        // Initialize Wi-Fi NVS
        let wifi_nvs = match EspNvs::new(nvs_default_partition.clone(), WIFI_NAMESPACE, true) {
            Ok(nvs) => {
                log::info!("Got namespace {WIFI_NAMESPACE} from default partition");
                nvs
            }
            Err(e) => panic!("Could't get wifi namespace {e:?}"),
        };

        // Initialize Timezone NVS
        let tz_nvs = match EspNvs::new(nvs_default_partition.clone(), TZ_NAMESPACE, true) {
            Ok(nvs) => {
                log::info!("Got namespace {TZ_NAMESPACE} from default partition");
                nvs
            }
            Err(e) => panic!("Could't get tz namespace {e:?}"),
        };

        let app_storage = Self { wifi_nvs, tz_nvs };

        Ok(SharedAppStorage::new(app_storage.into()))
    }
}
