use error::AppError;
use esp_idf_svc::{
    hal::{delay::FreeRtos, prelude::Peripherals},
    http::Method,
};

mod display;
mod error;
mod handler;
mod led;
mod server;
mod time;
mod utils;
mod wifi;

fn main() -> Result<(), AppError> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("Failed to take peripherals");

    // Initialize the display
    let display = display::get_display(peripherals.pins.gpio4, peripherals.pins.gpio5)
        .inspect_err(|e| {
            log::error!("Failed to get display: {:#?}", e);
        })?;
    display::init_display(&display).inspect_err(|e| {
        log::error!("Failed to initialize display: {:#?}", e);
    })?;

    // Initialize the led strip
    let led_strip = led::LedStrip::new(peripherals.rmt.channel0, peripherals.pins.gpio13, 7)
        .inspect_err(|e| {
            log::error!("Failed to get led strip: {:#?}", e);
        })?;
    led_strip.lock().unwrap().turn_off()?;

    // Initialize Wi-Fi
    let mut wifi = wifi::get_wifi(peripherals.modem).inspect_err(|e| {
        log::error!("Failed to get Wi-Fi: {:#?}", e);
    })?;
    wifi::init_wifi(&mut wifi).inspect_err(|e| {
        log::error!("Failed to initialize Wi-Fi: {:#?}", e);
    })?;

    // Initialize SNTP
    let sntp = time::get_sntp().inspect_err(|e| {
        log::error!("Failed to get SNTP: {:#?}", e);
    })?;
    time::init_sntp(&sntp).inspect_err(|e| {
        log::error!("Failed to initialize SNTP: {:#?}", e);
    })?;

    led_strip
        .lock()
        .unwrap()
        .set_theme(led::LedStripTheme::default())?;

    // Start the HTTP server
    let mut http_server = server::start_server().inspect_err(|e| {
        log::error!("Failed to start HTTP server: {:#?}", e);
    })?;

    // Define HTTP routes
    http_server
        .fn_handler("/", Method::Get, handler::index())
        .inspect_err(|&e| {
            log::error!("Failed to register index handler: {:#?}", e);
        })?;

    http_server
        .fn_handler("/get_status", Method::Get, handler::get_status())
        .inspect_err(|&e| {
            log::error!("Failed to register get_status handler: {:#?}", e);
        })?;

    unsafe {
        http_server
            .fn_handler_nonstatic(
                "/set_digits",
                Method::Get,
                handler::set_digits(display.clone()),
            )
            .inspect_err(|&e| {
                log::error!("Failed to register set_digits handler: {:#?}", e);
            })?;

        http_server
            .fn_handler_nonstatic(
                "/set_brightness",
                Method::Get,
                handler::set_brightness(display.clone()),
            )
            .inspect_err(|&e| {
                log::error!("Failed to register set_brightness handler: {:#?}", e);
            })?;

        http_server
            .fn_handler_nonstatic(
                "/sync_time",
                Method::Get,
                handler::sync_time(display.clone(), sntp),
            )
            .inspect_err(|&e| {
                log::error!("Failed to register sync_time handler: {:#?}", e);
            })?;

        http_server
            .fn_handler_nonstatic("/set_theme", Method::Get, handler::set_theme(led_strip))
            .inspect_err(|&e| {
                log::error!("Failed to register set_theme handler: {:#?}", e);
            })?;
    }

    // Create a thread for updating the time in display
    std::thread::spawn(move || loop {
        display::update_display_time(&display.clone())
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
