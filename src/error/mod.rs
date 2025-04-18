/// Represents errors that can occur in the application.
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Esp I/O error: {0}")]
    EspIO(#[from] esp_idf_svc::io::EspIOError),

    #[error("Std I/O error: {0}")]
    StdIO(#[from] std::io::Error),

    #[error("System error: {0}")]
    System(#[from] esp_idf_svc::sys::EspError),

    #[error("Display error: {0}")]
    Display(String),

    #[error("Led strip error: {0}")]
    LedStrip(String),

    #[error("Server error: {0}")]
    Server(String),
}

impl From<tm1637::Error<esp_idf_svc::sys::EspError>> for AppError {
    fn from(value: tm1637::Error<esp_idf_svc::sys::EspError>) -> Self {
        AppError::Display(format!("{:?}", value))
    }
}

impl From<ws2812_esp32_rmt_driver::Ws2812Esp32RmtDriverError> for AppError {
    fn from(value: ws2812_esp32_rmt_driver::Ws2812Esp32RmtDriverError) -> Self {
        AppError::LedStrip(format!("{:?}", value))
    }
}
