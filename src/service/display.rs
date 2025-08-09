use crate::{error::AppError, module::led::SharedAmPmIndicator, prefs::hour_format::HourFormat};
use esp_idf_svc::hal::gpio::OutputPin;

/// Defines the service for controlling a 7-segment display.
pub trait SevenSegmentDisplayService {
    fn init(&mut self) -> Result<(), AppError>;
    fn write(&mut self, message: [u8; 4]) -> Result<(), AppError>;
    fn set_brightness(&mut self, level: u8) -> Result<(), AppError>;
    fn update_display_hour<AM: OutputPin, PM: OutputPin>(
        &mut self,
        am_pm_indicator: SharedAmPmIndicator<AM, PM>,
        hour_format: HourFormat,
    ) -> Result<(), AppError>;
    fn update_display_year(&mut self) -> Result<(), AppError>;
    fn update_display_date(&mut self) -> Result<(), AppError>;
}
