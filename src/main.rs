use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{delay::FreeRtos, prelude::Peripherals},
    http::Method,
    nvs::{EspDefaultNvsPartition, EspNvs},
    sys::esp_restart,
};
use server::dns_responder::DnsResponder;
use std::{
    net::Ipv4Addr,
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
};
use wifi::ap::AP_IP_ADDRESS;

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
    // Define the namespace for storing Timezone settings in NVS
    let tz_namespace = "tz_ns";

    // Initialize Wi-Fi NVS
    let mut wifi_nvs = match EspNvs::new(nvs_default_partition.clone(), wifi_namespace, true) {
        Ok(nvs) => {
            log::info!("Got namespace {:?} from default partition", wifi_namespace);
            nvs
        }
        Err(e) => panic!("Could't get wifi namespace {:?}", e),
    };

    // Initialize Timezone NVS
    let mut tz_nvs = match EspNvs::new(nvs_default_partition.clone(), tz_namespace, true) {
        Ok(nvs) => {
            log::info!("Got namespace {:?} from default partition", tz_namespace);
            nvs
        }
        Err(e) => panic!("Could't get tz namespace {:?}", e),
    };

    let credentials = nvs::wifi::get_maybe_wifi_credentials(&mut wifi_nvs).unwrap();

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
        wifi::station::connect_wifi_or_restart(&mut wifi_station, &mut wifi_nvs)?;

        wifi_station
    };

    log::info!("Wi-Fi Config: {:?}", wifi.get_configuration().unwrap());

    // If the device is in AP mode, start the captive portal to capture credentials
    if is_ap_mode {
        let ap_ip_address = Ipv4Addr::from_str(AP_IP_ADDRESS).expect("Error reading AP_IP_ADDRESS");

        // Starts the DNS server for the Captive Portal
        log::info!("Starting DNS Responder...");
        let mut dns_responder =
            DnsResponder::init(ap_ip_address).expect("Failed to initialize DNS Responder");

        // Runs the DNS server on another thread and accepts the timeout error with
        // .ok().
        std::thread::spawn(move || loop {
            dns_responder.handle_requests().ok();
            std::thread::sleep(Duration::from_millis(100));
        });

        // Starts the server with the Wi-Fi configuration handler and the captive portal
        // redirection handlers
        server::captive_portal::start_captive_portal()?;

        // If new credentials are received, store them in NVS
        if let Some(credentials) = wifi::WIFI_CREDENTIALS.lock().unwrap().clone() {
            nvs::wifi::save_wifi_credentials(&mut wifi_nvs, credentials.ssid, credentials.password);
        }

        // Stop the AP Wi-Fi interface
        wifi.stop()?;

        // Restart the device after the configuration
        unsafe {
            esp_restart();
        }
    }

    // Initialize mDNS
    let mut mdns = esp_idf_svc::mdns::EspMdns::take()?;
    mdns.set_hostname("espclock")?;
    mdns.set_instance_name("espclock")?;
    mdns.add_service(None, "_http", "_tcp", 80, &[])?;

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

    // Read timezone from NVS
    let timezone = nvs::tz::get_maybe_timezone(&mut tz_nvs);

    if let Some(tz) = timezone.unwrap_or(None) {
        time::tz::set_timezone(tz);
    } else {
        time::tz::set_timezone(env!("DEFAULT_TIMEZONE").to_string());
    }

    let wifi_nvs = Arc::new(Mutex::new(wifi_nvs));
    let tz_nvs = Arc::new(Mutex::new(tz_nvs));

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
            log::error!("Failed to register web portal handler: {:#?}", e);
        })?;

    web_portal_server
        .fn_handler(
            "/assets/index.css",
            Method::Get,
            server::web_portal::web_portal_css(),
        )
        .inspect_err(|&e| {
            log::error!("Failed to serve CSS: {:#?}", e);
        })?;

    web_portal_server
        .fn_handler(
            "/assets/js/index.js",
            Method::Get,
            server::web_portal::web_portal_js(),
        )
        .inspect_err(|&e| {
            log::error!("Failed to serve JS: {:#?}", e);
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
            "/set_theme",
            Method::Get,
            server::web_portal::set_theme(led_strip),
        )
        .inspect_err(|&e| {
            log::error!("Failed to register set_theme handler: {:#?}", e);
        })?;

    web_portal_server
        .fn_handler(
            "/set_timezone",
            Method::Post,
            server::web_portal::set_timezone(tz_nvs.clone()),
        )
        .inspect_err(|&e| {
            log::error!("Failed to register set_timezone handler: {:#?}", e);
        })?;

    web_portal_server
        .fn_handler(
            "/factory_reset",
            Method::Get,
            server::web_portal::factory_reset(wifi_nvs, tz_nvs),
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
    }

    // Create a thread for updating the time in display
    std::thread::spawn(move || loop {
        let wait_time = time::calculate_time_until_next_minute();

        // Wait until the next minute
        FreeRtos::delay_ms(wait_time.as_millis() as u32);

        module::display::update_display_time(&display.clone())
            .inspect_err(|e| {
                log::error!("Failed to update display time: {:#?}", e);
            })
            .unwrap();
    });

    loop {
        FreeRtos::delay_ms(1000);
    }
}
