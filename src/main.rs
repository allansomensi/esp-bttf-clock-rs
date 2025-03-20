use std::sync::{Arc, Mutex};

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{delay::FreeRtos, prelude::Peripherals},
    http::Method,
    nvs::{EspDefaultNvsPartition, EspNvs},
    sys::esp_restart,
};

mod error;
mod module;
mod nvs;
mod server;
mod time;
mod util;
mod wifi;

fn main() -> Result<(), error::AppError> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("Failed to take peripherals");
    let sysloop = EspSystemEventLoop::take().expect("Failed to take event loop");
    let nvs_default_partition = EspDefaultNvsPartition::take()?;

    // Define the namespace for storing Wi-Fi settings in NVS
    let wifi_namespace = "wifi_ns";

    // Initialize NVS
    let mut nvs = match EspNvs::new(nvs_default_partition.clone(), wifi_namespace, true) {
        Ok(nvs) => {
            log::info!("Got namespace {:?} from default partition", wifi_namespace);
            nvs
        }
        Err(e) => panic!("Could't get namespace {:?}", e),
    };

    let credentials = nvs::get_maybe_wifi_credentials(&mut nvs).unwrap();

    let is_ap_mode: bool;

    // If no credentials are found, get the Access Point (AP) instance
    let mut wifi = if credentials.is_none() {
        is_ap_mode = true;

        log::warn!("Credentials not found. Starting Wifi Access Point...");

        // Initialize the Wi-Fi Access Point
        let mut wifi_ap = wifi::ap::get_ap(
            peripherals.modem,
            sysloop.clone(),
            Some(nvs_default_partition),
        )?;

        // Starts the AP
        wifi::ap::start_wifi_ap(&mut wifi_ap)?;

        wifi_ap
    } else {
        // If credentials are found, start the Station mode to connect to a network
        is_ap_mode = false;

        let credentials = credentials.unwrap();
        let ssid = credentials.ssid;
        let password = credentials.password;

        log::info!("Credentials found. Starting Wifi Station...");
        log::info!("Wi-Fi SSID: {ssid}");
        log::info!("WIFI PASS: {password}");

        // Initialize the Wi-Fi Station
        let mut wifi_station = wifi::station::get_station(
            peripherals.modem,
            sysloop.clone(),
            Some(nvs_default_partition),
            ssid,
            password,
        )?;

        // Connect to the Wi-Fi network
        wifi::station::connect_wifi(&mut wifi_station)?;

        wifi_station
    };

    log::info!("Wi-Fi Config: {:?}", wifi.get_configuration().unwrap());

    // If the device is in AP mode, start the captive portal to capture credentials
    if is_ap_mode {
        server::captive_portal::start_captive_portal()?;

        // If new credentials are received, store them in NVS
        if let Some(credentials) = wifi::WIFI_CREDENTIALS.lock().unwrap().clone() {
            nvs::save_wifi_credentials(&mut nvs, credentials.ssid, credentials.password);
        }

        // Stop the AP Wi-Fi interface
        wifi.stop()?;

        // Restart the device after the configuration
        unsafe {
            esp_restart();
        }
    }

    // Initialize the display
    let display = module::display::get_display(peripherals.pins.gpio4, peripherals.pins.gpio5)
        .inspect_err(|e| {
            log::error!("Failed to get display: {:#?}", e);
        })?;
    module::display::init_display(&display).inspect_err(|e| {
        log::error!("Failed to initialize display: {:#?}", e);
    })?;
    log::info!("Display initialized successfully!");

    // Initialize the led strip
    let led_strip =
        module::led::LedStrip::new(peripherals.rmt.channel0, peripherals.pins.gpio13, 7)
            .inspect_err(|e| {
                log::error!("Failed to get led strip: {:#?}", e);
            })?;
    led_strip.lock().unwrap().turn_off()?;
    log::info!("Led strip initialized successfully!");

    // Initialize SNTP
    let sntp = time::sntp::get_sntp().inspect_err(|e| {
        log::error!("Failed to get SNTP: {:#?}", e);
    })?;
    time::sntp::init_sntp(&sntp).inspect_err(|e| {
        log::error!("Failed to initialize SNTP: {:#?}", e);
    })?;

    // Set the LED strip theme to default
    led_strip
        .lock()
        .unwrap()
        .set_theme(module::led::LedStripTheme::default())?;

    // Start the Web portal HTTP server
    let mut web_portal_server = server::create_server().inspect_err(|e| {
        log::error!("Failed to start HTTP server: {:#?}", e);
    })?;

    // Define HTTP routes
    web_portal_server
        .fn_handler("/", Method::Get, server::web_portal::web_portal())
        .inspect_err(|&e| {
            log::error!("Failed to register index handler: {:#?}", e);
        })?;

    let wifi_ssid = wifi
        .wifi()
        .get_configuration()
        .unwrap()
        .as_client_conf_ref()
        .unwrap()
        .ssid
        .to_string();

    web_portal_server
        .fn_handler(
            "/get_status",
            Method::Get,
            server::web_portal::get_status(wifi_ssid),
        )
        .inspect_err(|&e| {
            log::error!("Failed to register get_status handler: {:#?}", e);
        })?;

    web_portal_server
        .fn_handler(
            "/factory_reset",
            Method::Get,
            server::web_portal::factory_reset(Arc::new(Mutex::new(nvs))),
        )
        .inspect_err(|&e| {
            log::error!("Failed to register sync_time handler: {:#?}", e);
        })?;

    unsafe {
        web_portal_server
            .fn_handler_nonstatic(
                "/set_digits",
                Method::Get,
                server::web_portal::set_digits(display.clone()),
            )
            .inspect_err(|&e| {
                log::error!("Failed to register set_digits handler: {:#?}", e);
            })?;

        web_portal_server
            .fn_handler_nonstatic(
                "/set_brightness",
                Method::Get,
                server::web_portal::set_brightness(display.clone()),
            )
            .inspect_err(|&e| {
                log::error!("Failed to register set_brightness handler: {:#?}", e);
            })?;

        web_portal_server
            .fn_handler_nonstatic(
                "/sync_time",
                Method::Get,
                server::web_portal::sync_time(display.clone(), sntp),
            )
            .inspect_err(|&e| {
                log::error!("Failed to register sync_time handler: {:#?}", e);
            })?;

        web_portal_server
            .fn_handler_nonstatic(
                "/set_theme",
                Method::Get,
                server::web_portal::set_theme(led_strip),
            )
            .inspect_err(|&e| {
                log::error!("Failed to register set_theme handler: {:#?}", e);
            })?;
    }

    // Create a thread for updating the time in display
    std::thread::spawn(move || loop {
        module::display::update_display_time(&display.clone())
            .inspect_err(|e| {
                log::error!("Failed to update display time: {:#?}", e);
            })
            .unwrap();
        FreeRtos::delay_ms(60000);
    });

    loop {
        FreeRtos::delay_ms(1000);
    }
}
