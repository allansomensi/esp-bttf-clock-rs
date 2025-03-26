use crate::{
    error::AppError,
    module::{
        display::{self, DisplayMessage, SharedTm1637},
        led::{LedStripTheme, SharedLedStrip},
    },
    nvs,
    time::{self, tz::TimezoneRequest},
    util,
};
use chrono_tz::Tz;
use esp_idf_svc::{
    hal::gpio::{IOPin, OutputPin},
    http::server::{EspHttpConnection, Request},
    io::Write,
    nvs::{EspNvs, NvsDefault},
    sntp::{EspSntp, SyncStatus},
    sys::{esp_restart, esp_wifi_disconnect, sntp_restart},
};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

static WEB_PORTAL_HTML: &str = include_str!("../../web/web_portal/dist/index.html");
static WEB_PORTAL_CSS: &str = include_str!("../../web/web_portal/dist/assets/index.css");
static WEB_PORTAL_JS: &str = include_str!("../../web/web_portal/dist/assets/js/index.js");

/// Generates the web portal page response for the HTTP request.
///
/// This function returns a closure that handles the HTTP request for the
/// `web_portal.html` page. It serves the contents of an HTML file as the
/// response.
///
/// ## Returns
///
/// A closure that handles an HTTP request and returns an HTML response
/// with the content of the `web_portal.html` file.
pub fn web_portal() -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> {
    move |request: Request<&mut EspHttpConnection<'_>>| {
        request
            .into_ok_response()?
            .write_all(WEB_PORTAL_HTML.as_bytes())?;
        Ok::<(), AppError>(())
    }
}

/// Serves the CSS file for the web portal.
pub fn web_portal_css() -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> {
    move |request: Request<&mut EspHttpConnection<'_>>| {
        request
            .into_response(200, None, &[("Content-Type", "text/css; charset=utf-8")])?
            .write_all(WEB_PORTAL_CSS.as_bytes())?;
        Ok::<(), AppError>(())
    }
}

/// Serves the JavaScript file for the web portal.
pub fn web_portal_js() -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> {
    move |request: Request<&mut EspHttpConnection<'_>>| {
        request
            .into_response(
                200,
                None,
                &[("Content-Type", "application/javascript; charset=utf-8")],
            )?
            .write_all(WEB_PORTAL_JS.as_bytes())?;
        Ok::<(), AppError>(())
    }
}

/// Returns the current status of the system including Wi-Fi SSID, Timezone and
/// actual time.
///
/// ## Returns
///
/// A closure that handles the HTTP request and returns an HTML response with
/// system status information.
pub fn get_status(
    wifi_ssid: String,
) -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> {
    move |request: Request<&mut EspHttpConnection<'_>>| {
        let timezone = time::tz::get_timezone();
        let time = time::get_time();
        let wifi_ssid = wifi_ssid.as_str();

        let status_html = format!(
            "<p><strong>Wi-Fi SSID:</strong> {wifi_ssid}</p>
        <p><strong>Time Zone:</strong> {timezone}</p>
        <p><strong>Current Time:</strong> {}{}:{}{}</p>",
            time[0], time[1], time[2], time[3]
        );

        request.into_ok_response()?.write(status_html.as_bytes())?;

        Ok::<(), AppError>(())
    }
}

/// Sets the timezone based on the timezone data from the request body.
///
/// This function extracts the timezone information from the incoming request,
/// validates the format, and updates the timezone accordingly. It also
/// saves the timezone data in NVS and responds with a success message.
///
/// ## Arguments
///
/// * `tz_nvs` - A [Mutex] wrapping the [EspNvs] instance for storing the
///   timezone data.
///
/// ## Returns
///
/// A closure that handles the HTTP request, validates the timezone, updates the
/// system timezone, saves the data in NVS, and responds with a success message.
pub fn set_timezone(
    tz_nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
) -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> {
    move |mut request: Request<&mut EspHttpConnection<'_>>| {
        let mut buf = [0u8; 128];
        let len = request.read(&mut buf)?;
        let buf = &buf[..len];

        let timezone_data: TimezoneRequest = match serde_json::from_slice(buf) {
            Ok(data) => data,
            Err(_) => {
                log::error!("Invalid JSON format");
                request.into_status_response(400)?;
                return Err(AppError::Server("Invalid request".to_string()));
            }
        };

        if Tz::from_str(&timezone_data.timezone).is_err() {
            log::error!("Invalid timezone: {}", timezone_data.timezone);
            request.into_status_response(400)?;
            return Err(AppError::Server("Invalid request".to_string()));
        }

        let mut tz_lock = tz_nvs.lock().unwrap();
        nvs::tz::save_timezone(&mut tz_lock, timezone_data.clone());
        time::tz::set_timezone(timezone_data.timezone);

        request
            .into_ok_response()?
            .write("Timezone changed!".as_bytes())?;

        Ok::<(), AppError>(())
    }
}

/// Creates an HTTP handler that performs a factory reset by deleting Wi-Fi
/// credentials and restarting the device.
///
/// ## Behavior
///
/// - Deletes the stored Wi-Fi credentials from NVS.
/// - Deletes the stored Timezone settings from NVS.
/// - Disconnects from the current Wi-Fi network.
/// - Restarts the ESP32 device.
///
/// ## Returns
///
/// - A closure that can be used as an HTTP request handler.
/// - This function does not return control after execution, as the device
///   restarts.
///
/// ## Safety
///
/// - Calls `esp_restart()`, which immediately reboots the device. Any unsaved
///   data will be lost.
pub fn factory_reset(
    wifi_nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
    tz_nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
) -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> {
    move |_: Request<&mut EspHttpConnection<'_>>| {
        nvs::wifi::delete_wifi_credentials(&mut wifi_nvs.lock().unwrap());
        nvs::tz::delete_timezone(&mut tz_nvs.lock().unwrap());
        log::info!("Factory reset initiated!");
        log::info!("Restarting...");

        unsafe {
            esp_wifi_disconnect();
            esp_restart();
        }
    }
}

/// Updates the display digits based on the digits found in the request URL.
///
/// This function retrieves four digits from the request URI, updates the
/// display accordingly, and responds with a success message.
///
/// ## Arguments
///
/// * `display` - A [SharedTm1637] display instance.
///
/// ## Returns
///
/// A closure that handles the HTTP request, updates the display, and returns
/// a success message.
pub unsafe fn set_digits(
    display: SharedTm1637<impl OutputPin, impl IOPin>,
) -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> {
    move |request: Request<&mut EspHttpConnection<'_>>| {
        let digits = util::find_digits_in_url(request.uri());

        let mut locked_display = display.lock().unwrap();
        locked_display.clear()?;
        locked_display.print_hex(0, &digits)?;

        log::info!("Display digits updated manually");

        request
            .into_ok_response()?
            .write("Digits inserted!".as_bytes())?;

        Ok::<(), AppError>(())
    }
}

/// Sets the brightness of the display based on the request URL.
///
/// This function extracts the brightness value from the URL query parameters
/// and updates the display's brightness accordingly. The brightness value must
/// be between 0 and 7.
///
/// ## Arguments
///
/// * `display` - A [SharedTm1637] display instance.
///
/// ## Returns
///
/// A closure that handles the HTTP request, updates the brightness, and returns
/// a success message.
pub unsafe fn set_brightness(
    display: SharedTm1637<impl OutputPin, impl IOPin>,
) -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> {
    move |request: Request<&mut EspHttpConnection<'_>>| {
        let url = request.uri();

        if let Some(start) = url.find('?') {
            let brightness_value = &url[start + 1..];
            if let Ok(brightness) = brightness_value.parse::<u8>() {
                if (0..=7).contains(&brightness) {
                    display.lock().unwrap().set_brightness(brightness)?;
                    log::info!("Brightness updated to level {brightness}");
                }
            }
        }

        request
            .into_ok_response()?
            .write("Brightness Updated!".as_bytes())?;

        Ok::<(), AppError>(())
    }
}

/// Synchronizes the system time using SNTP and updates the display with the
/// sync message.
///
/// This function restarts the SNTP synchronization process, waits for
/// completion, and updates the display with the current time once
/// synchronization is finished.
///
/// ## Arguments
///
/// * `display` - A [SharedTm1637] display instance.
/// * `sntp` - An instance of [Sntp] used to synchronize the time.
///
/// ## Returns
///
/// A closure that handles the HTTP request, synchronizes the time, updates the
/// display, and returns a success message.
pub unsafe fn sync_time(
    display: SharedTm1637<impl OutputPin, impl IOPin>,
    sntp: EspSntp<'static>,
) -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> {
    move |request: Request<&mut EspHttpConnection<'_>>| {
        let sync_message = DisplayMessage::Sync.as_bytes();

        sntp_restart();

        log::info!("Synchronizing with SNTP Server");

        display::write(&display, sync_message)?;

        while sntp.get_sync_status() != SyncStatus::Completed {}
        display::update_display_time(&display)?;

        log::info!("Time sync completed!");

        request
            .into_ok_response()?
            .write("Time synced successfully!".as_bytes())?;

        Ok::<(), AppError>(())
    }
}

/// Creates an HTTP handler that changes the LED strip theme based on a query
/// parameter.
///
/// ## Arguments
///
/// - Reads the requested theme from the URL query parameter.
/// - Sets the LED strip color based on the provided theme value.
/// - Responds with `"Theme Updated!"` if successful.
/// - Returns an error if the theme value is invalid.
///
/// ## Returns
///
/// - A closure that acts as an HTTP request handler.
pub fn set_theme(
    led_strip: SharedLedStrip,
) -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> {
    move |request: Request<&mut EspHttpConnection<'_>>| {
        let url = request.uri();

        if let Some(start) = url.find('?') {
            let theme_value = &url[start + 1..];

            match theme_value {
                "orange" => {
                    led_strip.lock().unwrap().set_theme(LedStripTheme::Orange)?;
                    log::info!("Theme changed to Orange");
                }
                "blue" => {
                    led_strip.lock().unwrap().set_theme(LedStripTheme::Blue)?;
                    log::info!("Theme changed to Blue");
                }
                "green" => {
                    led_strip.lock().unwrap().set_theme(LedStripTheme::Green)?;
                    log::info!("Theme changed to Green");
                }
                _ => {
                    log::warn!("Invalid theme: {theme_value}");
                    return Err(AppError::Server("Invalid request".to_string()));
                }
            }
        }

        request
            .into_ok_response()?
            .write("Theme Updated!".as_bytes())?;

        Ok::<(), AppError>(())
    }
}
