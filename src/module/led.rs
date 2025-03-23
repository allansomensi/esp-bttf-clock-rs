use crate::error::AppError;
use esp_idf_svc::hal::{gpio::IOPin, peripheral::Peripheral, rmt::RmtChannel};
use std::sync::{Arc, Mutex};
use ws2812_esp32_rmt_driver::{Ws2812Esp32Rmt, RGB8};

/// Enum representing predefined LED strip color themes.
pub enum LedStripTheme {
    Orange,
    Blue,
    Green,
}

impl Default for LedStripTheme {
    fn default() -> Self {
        Self::Orange
    }
}

impl LedStripTheme {
    /// Generates a vector of LED colors based on the selected theme.
    ///
    /// ## Arguments
    /// - `num_leds`: The number of LEDs in the strip.
    ///
    /// ## Returns
    /// A vector containing `num_leds` elements of the selected theme color.
    pub fn get_colors(&self, num_leds: u8) -> Vec<RGB8> {
        let color = match self {
            LedStripTheme::Orange => RGB8 { r: 255, g: 0, b: 0 },
            LedStripTheme::Blue => RGB8 { r: 0, g: 0, b: 255 },
            LedStripTheme::Green => RGB8 { r: 0, g: 255, b: 0 },
        };
        vec![color; num_leds as usize]
    }
}

/// Type alias for a shared [LedStrip] instance.
///
/// This is an `Arc<Mutex<>>` to ensure thread safety and shared access to the
/// LED strip.
pub type SharedLedStrip = Arc<Mutex<LedStrip<'static>>>;

/// Struct representing a WS2812 LED strip.
pub struct LedStrip<'a> {
    ws2812: Ws2812Esp32Rmt<'a>,
    pub num_leds: u8,
}

impl LedStrip<'_> {
    /// Creates a new [LedStrip] instance.
    ///
    /// ## Arguments
    /// - `channel`: The RMT channel to use for LED communication (implements
    ///   [Peripheral] + [RmtChannel]).
    /// - `dio`: The data pin for the LED strip (implements [IOPin]).
    /// - `num_leds`: The number of LEDs in the strip.
    ///
    /// ## Returns
    /// A `Result` containing a shared [LedStrip] instance on success, or an
    /// [AppError] on failure.
    ///
    /// ## Example
    /// ```
    /// let led_strip = LedStrip::new(channel, dio, 7).expect("Failed to create LED strip");
    /// ```
    pub fn new<C, DIO>(channel: C, dio: DIO, num_leds: u8) -> Result<SharedLedStrip, AppError>
    where
        C: Peripheral<P = C> + RmtChannel + 'static,
        DIO: IOPin,
    {
        let ws2812 = Ws2812Esp32Rmt::new(channel, dio)?;
        Ok(Arc::new(Mutex::new(LedStrip { ws2812, num_leds })))
    }

    /// Turns off all LEDs in the strip.
    ///
    /// ## Returns
    /// A `Result` indicating success or an [AppError] on failure.
    pub fn turn_off(&mut self) -> Result<(), AppError> {
        let data = vec![RGB8 { r: 0, g: 0, b: 0 }; self.num_leds as usize];
        self.ws2812.write_nocopy(data)?;
        Ok(())
    }

    /// Sets the LED strip to a predefined color theme.
    ///
    /// # Arguments
    /// - `theme`: The [LedStripTheme] to apply to the LEDs.
    ///
    /// # Returns
    /// A `Result` indicating success or an `AppError` on failure.
    pub fn set_theme(&mut self, theme: LedStripTheme) -> Result<(), AppError> {
        let data = theme.get_colors(self.num_leds);
        self.ws2812.write_nocopy(data)?;
        Ok(())
    }
}
