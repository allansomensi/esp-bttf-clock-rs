use crate::{
    error::AppError,
    service::led_strip::LedStripService,
    theme::{AppTheme, Theme},
};
use esp_idf_svc::hal::{gpio::OutputPin, peripheral::Peripheral, rmt::RmtChannel};
use std::sync::{Arc, Mutex};
use ws2812_esp32_rmt_driver::{Ws2812Esp32Rmt, RGB8};

/// Type alias for a shared [LedStrip] instance.
///
/// This is an `Arc<Mutex<>>` to ensure thread safety and shared access to the
/// LED strip.
pub type SharedLedStrip = Arc<Mutex<LedStrip<'static>>>;

impl AppTheme for LedStrip<'_> {
    /// Sets the LED strip to a predefined color theme.
    ///
    /// ## Arguments
    /// - `theme`: The [LedStripTheme] to apply to the LEDs.
    ///
    /// ## Returns
    /// A `Result` indicating success or an `AppError` on failure.
    fn apply_theme(&mut self, theme: &Theme) -> Result<(), AppError> {
        let brightness = 0.1;

        let color = match theme {
            Theme::Orange => RGB8 {
                r: (255.0 * brightness) as u8,
                g: (0.0 * brightness) as u8,
                b: (0.0 * brightness) as u8,
            },
            Theme::Green => RGB8 {
                r: (0.0 * brightness) as u8,
                g: (255.0 * brightness) as u8,
                b: (0.0 * brightness) as u8,
            },
            Theme::Blue => RGB8 {
                r: (0.0 * brightness) as u8,
                g: (0.0 * brightness) as u8,
                b: (255.0 * brightness) as u8,
            },
        };

        let data = vec![color; self.num_leds as usize];

        self.ws2812.lock().unwrap().write_nocopy(data)?;
        Ok(())
    }
}

/// Struct representing a WS2812 LED strip.
pub struct LedStrip<'a> {
    ws2812: Arc<Mutex<Ws2812Esp32Rmt<'a>>>,
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
    pub fn new<C, DIO>(channel: C, dio: DIO, num_leds: u8) -> Result<Self, AppError>
    where
        C: Peripheral<P = C> + RmtChannel + 'static,
        DIO: OutputPin,
    {
        let ws2812 = Ws2812Esp32Rmt::new(channel, dio)?;
        let led_strip = LedStrip {
            ws2812: Arc::new(Mutex::new(ws2812)),
            num_leds,
        };
        Ok(led_strip)
    }
}

impl LedStripService for LedStrip<'_> {
    fn init(&mut self) -> Result<(), AppError> {
        self.turn_off()?;
        log::info!("Led strip initialized successfully!");

        Ok(())
    }

    /// Turns off all LEDs in the strip.
    ///
    /// ## Returns
    /// A `Result` indicating success or an [AppError] on failure.
    fn turn_off(&mut self) -> Result<(), AppError> {
        let data = vec![RGB8 { r: 0, g: 0, b: 0 }; self.num_leds as usize];
        self.ws2812.lock().unwrap().write_nocopy(data)?;
        Ok(())
    }
}
