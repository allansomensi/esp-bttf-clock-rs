/// Represents the different visual themes available for the LED strip.
#[derive(Default)]
pub enum Theme {
    /// Inspired by the official logo.
    #[default]
    Original,
    /// Inspired by Marty's iconic hoverboard.
    Hoverboard,
    /// Referencing the DeLorean's original fuel.
    Plutonium,
    /// Evokes the rustic, sepia-toned era of Back to the Future Part III.
    OldWest,
    /// A retro, neon-soaked palette reminiscent of the 2015 "Cafe 80s" diner.
    Cafe80s,
}

/// Defines the capability for a component to apply a visual theme.
pub trait AppTheme {
    fn apply_theme(&mut self, theme: &Theme) -> Result<(), crate::error::AppError>;
}
