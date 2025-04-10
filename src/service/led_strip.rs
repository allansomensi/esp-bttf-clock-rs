use crate::error::AppError;

pub trait LedStripService {
    fn init(&mut self) -> Result<(), AppError>;
    fn turn_off(&mut self) -> Result<(), AppError>;
}
