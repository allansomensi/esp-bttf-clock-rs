use crate::error::AppError;

/// Defines the service for controlling an LED strip.
pub trait LedStripService {
    fn init(&mut self) -> Result<(), AppError>;
    fn turn_off(&mut self) -> Result<(), AppError>;
}
