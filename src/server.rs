use crate::error::AppError;
use esp_idf_svc::http::server::{Configuration as ServerConfiguration, EspHttpServer};

/// Initializes and starts an HTTP server.
///
/// This function creates a new instance of the [EspHttpServer] using the default configuration
/// provided by [ServerConfiguration::default]. It is used to set up a basic HTTP server that can
/// handle incoming requests.
///
/// # Returns
/// - `Ok(EspHttpServer)`: The successfully created HTTP server instance.
/// - `Err(AppError)`: If there is an error during server initialization.
///
/// # Example
/// ```rust
/// let server = start_server().expect("Failed to start HTTP server");
/// ```
pub fn start_server() -> Result<EspHttpServer<'static>, AppError> {
    let http_server = EspHttpServer::new(&ServerConfiguration::default())?;

    Ok(http_server)
}
