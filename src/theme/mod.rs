pub enum Theme {
    Orange,
    Blue,
    Green,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Orange
    }
}

pub trait AppTheme {
    fn apply_theme(&mut self, theme: &Theme) -> Result<(), crate::error::AppError>;
}
