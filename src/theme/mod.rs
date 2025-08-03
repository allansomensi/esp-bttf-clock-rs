pub enum Theme {
    Original,
    Hoverboard,
    Plutonium,
    OldWest,
    Cafe80s,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Original
    }
}

pub trait AppTheme {
    fn apply_theme(&mut self, theme: &Theme) -> Result<(), crate::error::AppError>;
}
