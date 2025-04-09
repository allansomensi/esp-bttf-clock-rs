use crate::error::AppError;

pub trait AmPmIndicatorService {
    fn set_am(&mut self) -> Result<(), AppError>;
    fn set_pm(&mut self) -> Result<(), AppError>;
    fn clear(&mut self) -> Result<(), AppError>;
}
