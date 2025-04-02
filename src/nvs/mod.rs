use crate::error::AppError;
use esp_idf_svc::nvs::{EspNvs, EspNvsPartition, NvsDefault};
use std::sync::{Arc, Mutex};
use tz::TZ_NAMESPACE;
use wifi::WIFI_NAMESPACE;

pub mod tz;
pub mod wifi;

pub type SharedNvs = Arc<Mutex<EspNvs<NvsDefault>>>;

pub struct AppStorage {
    pub wifi_nvs: SharedNvs,
    pub tz_nvs: SharedNvs,
}

impl AppStorage {
    pub fn new(nvs_default_partition: EspNvsPartition<NvsDefault>) -> Result<Self, AppError> {
        // Initialize Wi-Fi NVS
        let wifi_nvs = Arc::new(Mutex::new(
            match EspNvs::new(nvs_default_partition.clone(), WIFI_NAMESPACE, true) {
                Ok(nvs) => {
                    log::info!("Got namespace {WIFI_NAMESPACE} from default partition");
                    nvs
                }
                Err(e) => panic!("Could't get wifi namespace {:?}", e),
            },
        ));

        // Initialize Timezone NVS
        let tz_nvs = Arc::new(Mutex::new(
            match EspNvs::new(nvs_default_partition.clone(), TZ_NAMESPACE, true) {
                Ok(nvs) => {
                    log::info!("Got namespace {TZ_NAMESPACE} from default partition");
                    nvs
                }
                Err(e) => panic!("Could't get tz namespace {:?}", e),
            },
        ));

        Ok(Self { wifi_nvs, tz_nvs })
    }
}
