use crate::error::AppError;
use esp_idf_svc::http::server::{Configuration as ServerConfiguration, EspHttpServer};

pub mod captive_portal;
pub mod dns_responder;
pub mod web_portal;

/// Need lots of stack to parse JSON
const STACK_SIZE: usize = 10240;

/// Initializes and starts an HTTP server.
///
/// This function creates a new instance of the [EspHttpServer] using the default configuration
/// provided by [ServerConfiguration::default]. It is used to set up a basic HTTP server that can
/// handle incoming requests.
///
/// ## Returns
/// - `Ok(EspHttpServer)`: The successfully created HTTP server instance.
/// - `Err(AppError)`: If there is an error during server initialization.
///
/// ## Example
/// ```rust
/// let server = start_server().expect("Failed to start HTTP server");
/// ```
pub fn create_server() -> Result<EspHttpServer<'static>, AppError> {
    let server_configuration = ServerConfiguration {
        stack_size: STACK_SIZE,
        ..Default::default()
    };

    Ok(EspHttpServer::new(&server_configuration)?)
}
