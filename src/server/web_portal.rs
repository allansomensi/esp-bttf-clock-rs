use crate::{
    error::AppError,
    module::{
        display::SharedSevenSegmentDisplay,
        led::SharedAmPmIndicator,
        led_strip::{LedStrip, SharedLedStrip},
    },
    nvs::SharedAppStorage,
    service::{
        app_storage::{AppStorageTzService, AppStorageWifiService},
        display::SevenSegmentDisplayService,
    },
    theme::{AppTheme, Theme},
    time::{self, tz::TimezoneRequest},
    util::messages::DisplayMessage,
};
use chrono_tz::Tz;
use esp_idf_svc::{
    hal::gpio::{IOPin, OutputPin},
    http::{
        server::{EspHttpConnection, EspHttpServer, Request},
        Method,
    },
    io::Write,
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

pub struct WebPortal {
    server: EspHttpServer<'static>,
}

impl WebPortal {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            server: super::create_server().inspect_err(|e| {
                log::error!("Failed to start HTTP server: {e:#?}");
            })?,
        })
    }

    pub fn create_routes<CLK: OutputPin, DIO: IOPin, AM: OutputPin, PM: OutputPin>(
        &mut self,
        display: SharedSevenSegmentDisplay<'static, CLK, DIO>,
        am_pm_indicator: SharedAmPmIndicator<'static, AM, PM>,
        led_strip: LedStrip<'static>,
        app_storage: SharedAppStorage,
        sntp: EspSntp<'static>,
        wifi_ssid: String,
    ) -> Result<(), AppError> {
        self.server
            .fn_handler("/", Method::Get, web_portal())
            .inspect_err(|&e| {
                log::error!("Failed to register web portal handler: {e:#?}");
            })?;

        self.server
            .fn_handler("/assets/index.css", Method::Get, web_portal_css())
            .inspect_err(|&e| {
                log::error!("Failed to serve CSS: {e:#?}");
            })?;

        self.server
            .fn_handler("/assets/js/index.js", Method::Get, web_portal_js())
            .inspect_err(|&e| {
                log::error!("Failed to serve JS: {e:#?}");
            })?;

        self.server
            .fn_handler("/get_status", Method::Get, get_status(wifi_ssid))
            .inspect_err(|&e| {
                log::error!("Failed to register get_status handler: {e:#?}");
            })?;

        self.server
            .fn_handler(
                "/set_theme",
                Method::Get,
                set_theme(Arc::new(Mutex::new(led_strip))),
            )
            .inspect_err(|&e| {
                log::error!("Failed to register set_theme handler: {e:#?}");
            })?;

        self.server
            .fn_handler(
                "/set_timezone",
                Method::Post,
                set_timezone(app_storage.clone()),
            )
            .inspect_err(|&e| {
                log::error!("Failed to register set_timezone handler: {e:#?}");
            })?;

        self.server
            .fn_handler(
                "/factory_reset",
                Method::Get,
                factory_reset(app_storage.clone()),
            )
            .inspect_err(|&e| {
                log::error!("Failed to register sync_time handler: {e:#?}");
            })?;

        self.server
            .fn_handler(
                "/set_brightness",
                Method::Get,
                set_brightness(display.clone()),
            )
            .inspect_err(|&e| {
                log::error!("Failed to register set_brightness handler: {e:#?}");
            })?;

        self.server
            .fn_handler(
                "/sync_time",
                Method::Get,
                sync_time(display.clone(), am_pm_indicator.clone(), sntp),
            )
            .inspect_err(|&e| {
                log::error!("Failed to register sync_time handler: {e:#?}");
            })?;

        Ok(())
    }
}

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
        let time = time::get_hour_min();
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
    storage: SharedAppStorage,
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

        storage
            .lock()
            .unwrap()
            .save_timezone(timezone_data.clone())?;
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
pub fn factory_reset(
    storage: SharedAppStorage,
) -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> {
    move |_: Request<&mut EspHttpConnection<'_>>| {
        storage.lock().unwrap().delete_wifi_credentials()?;
        storage.lock().unwrap().delete_timezone()?;
        log::info!("Factory reset initiated!");
        log::info!("Restarting...");

        unsafe {
            esp_wifi_disconnect();
            esp_restart();
        }
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
pub fn set_brightness<'a, CLK, DIO>(
    display: SharedSevenSegmentDisplay<'a, CLK, DIO>,
) -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> + Send + 'a
where
    CLK: OutputPin + 'a,
    DIO: IOPin + 'a,
{
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
pub fn sync_time<'a, CLK, DIO, AM, PM>(
    display: SharedSevenSegmentDisplay<'a, CLK, DIO>,
    am_pm_indicator: SharedAmPmIndicator<'a, AM, PM>,
    sntp: EspSntp<'static>,
) -> impl Fn(Request<&mut EspHttpConnection<'_>>) -> Result<(), AppError> + Send + 'a
where
    CLK: OutputPin + 'a,
    DIO: IOPin + 'a,
    AM: OutputPin,
    PM: OutputPin,
{
    move |request: Request<&mut EspHttpConnection<'_>>| {
        let sync_message = DisplayMessage::Sync.as_bytes();

        unsafe {
            sntp_restart();
        }

        log::info!("Synchronizing with SNTP Server");

        display.lock().unwrap().write(sync_message)?;

        while sntp.get_sync_status() != SyncStatus::Completed {}
        display
            .lock()
            .unwrap()
            .update_display_hour(am_pm_indicator.clone())?;

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
                    led_strip.lock().unwrap().apply_theme(&Theme::Orange)?;
                    log::info!("Theme changed to Orange");
                }
                "green" => {
                    led_strip.lock().unwrap().apply_theme(&Theme::Green)?;
                    log::info!("Theme changed to Green");
                }
                "blue" => {
                    led_strip.lock().unwrap().apply_theme(&Theme::Blue)?;
                    log::info!("Theme changed to Blue");
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
