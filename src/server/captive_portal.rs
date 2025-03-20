use super::create_server;
use crate::{
    error::AppError,
    wifi::{WifiCredentials, WIFI_CREDENTIALS},
};
use embedded_svc::http::Headers;
use esp_idf_svc::{
    http::Method,
    io::{Read, Write},
};

/// Max payload length
const MAX_LEN: usize = 128;

static CAPTIVE_PORTAL_HTML: &str = include_str!("../view/captive_portal.html");

/// Starts a captive portal HTTP server for configuring Wi-Fi credentials.
///
/// ## Behavior
///
/// - Serves an HTML page at `"/"` to allow users to enter Wi-Fi credentials.
/// - Accepts a JSON payload via `POST /set_config` containing Wi-Fi credentials.
/// - Stores the received credentials in the [WIFI_CREDENTIALS] global variable.
/// - Waits until credentials are received before exiting.
///
/// ## Returns
///
/// - `Ok(())` if the portal is successfully initialized and credentials are received.
/// - `Err(AppError)` if server creation fails.
///
/// ## Example
///
/// ```rust
/// if let Err(e) = start_captive_portal() {
///     eprintln!("Failed to start captive portal: {:?}", e);
/// }
/// ```
pub fn start_captive_portal() -> Result<(), AppError> {
    let mut server = create_server()?;

    server.fn_handler("/", Method::Get, |req| {
        req.into_ok_response()?
            .write_all(CAPTIVE_PORTAL_HTML.as_bytes())
            .map(|_| ())
    })?;

    server.fn_handler::<AppError, _>("/set_config", Method::Post, |mut req| {
        let len = req.content_len().unwrap_or(0) as usize;

        if len > MAX_LEN {
            req.into_status_response(413)?
                .write_all("Request too big".as_bytes())?;
            return Ok(());
        }

        let mut buf = vec![0; len];
        req.read_exact(&mut buf).expect("Error in 'read_exact()'");
        let mut resp = req.into_ok_response()?;

        if let Ok(form) = serde_json::from_slice::<WifiCredentials>(&buf) {
            let mut credentials = WIFI_CREDENTIALS.lock().unwrap();
            *credentials = Some(form.clone());

            write!(
                resp,
                "SSID = {} and PASSWORD = {}",
                form.ssid, form.password
            )
            .expect("Error in 'write'");
        } else {
            resp.write_all("JSON error".as_bytes())?;
        }

        Ok(())
    })?;

    while WIFI_CREDENTIALS.lock().unwrap().is_none() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}
