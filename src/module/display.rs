use crate::{error::AppError, time, util::DISPLAY_DIGIT};
use esp_idf_svc::hal::{
    delay::Ets,
    gpio::{IOPin, InputOutput, Output, OutputPin, PinDriver},
};
use std::sync::{Arc, Mutex};
use tm1637::TM1637;

/// Type alias for the [TM1637] display using pin drivers and [Ets] as time
/// control. This is an ´Arc<Mutex<>>´ to ensure thread safety and shared access
/// to the display.
pub type SharedTm1637<'a, CLK, DIO> =
    Arc<Mutex<TM1637<'a, PinDriver<'a, CLK, Output>, PinDriver<'a, DIO, InputOutput>, Ets>>>;

/// A thread-safe shared `SevenSegmentDisplay` using `Arc<Mutex<...>>`.
pub type SharedSevenSegmentDisplay<'a, CLK, DIO> = Arc<Mutex<SevenSegmentDisplay<'a, CLK, DIO>>>;

/// Enum representing different display messages.
/// Used to send specific byte patterns to the display.
pub enum DisplayMessage {
    Init,
    Sync,
}

impl DisplayMessage {
    /// Converts the display message into a 4-byte array that represents the
    /// bits to be shown on the display.
    ///
    /// ## Returns
    /// Returns a 4-byte array, each byte representing a value for display
    /// output.
    ///
    /// ## Example
    /// ```rust
    /// let message = DisplayMessage::Init.as_bytes();
    /// ```
    pub fn as_bytes(&self) -> [u8; 4] {
        match self {
            DisplayMessage::Init => [
                0b00000110, // i
                0b01010100, // n
                0b00000100, // i
                0b01111000, // t
            ],
            DisplayMessage::Sync => [
                0b01101101, // s
                0b01101110, // y
                0b00110111, // n
                0b00111001, // c
            ],
        }
    }
}

/// Centralizes the logic for controlling a seven-segment display.
pub struct SevenSegmentDisplay<'a, CLK: OutputPin, DIO: IOPin> {
    tm1637: SharedTm1637<'a, CLK, DIO>,
}

impl<CLK, DIO> SevenSegmentDisplay<'_, CLK, DIO>
where
    CLK: OutputPin,
    DIO: IOPin,
{
    /// Creates a new `SevenSegmentDisplay` instance.
    ///
    /// Creates the TM1637-based seven-segment display by
    /// setting up the clock (CLK) and data (DIO) pins, configuring them
    /// appropriately, and creating a thread-safe shared instance
    /// of the underlying `TM1637` driver. The function ensures proper memory
    /// management by leaking the boxed pin and delay drivers to provide
    /// static references required by the `TM1637` struct.
    ///
    /// ## Arguments
    /// - `clk`: The GPIO pin used for the clock signal.
    /// - `dio`: The GPIO pin used for the data signal.
    ///
    /// ## Returns
    /// - `Ok(Self)`: A new instance of [SevenSegmentDisplay] if initialization
    ///   succeeds.
    /// - `Err(AppError)`: An error if any of the GPIO operations fail.
    ///
    /// ## Example
    /// ```rust
    /// let display =
    ///     SevenSegmentDisplay::new(clk_pin, dio_pin).expect("Failed to initialize the display");
    /// ```
    pub fn new(clk: CLK, dio: DIO) -> Result<Self, AppError> {
        let clk = Box::new(PinDriver::output(clk)?);
        let dio = Box::new(PinDriver::input_output(dio)?);
        let delay = Box::new(Ets);

        let display = Arc::new(Mutex::new(TM1637::new(
            Box::leak(clk),
            Box::leak(dio),
            Box::leak(delay),
        )));

        Ok(Self { tm1637: display })
    }

    /// Initializes the [SevenSegmentDisplay] by setting up the display and
    /// configuring the brightness.
    ///
    /// This function locks the underlying `TM1637` driver to initialize the
    /// display and set its brightness to a predefined level. It also writes
    /// an initialization message to the display to signal that the setup is
    /// complete.
    ///
    /// ## Returns
    /// - `Ok(())`: If the display is successfully initialized and the message
    ///   is written.
    /// - `Err(AppError)`: An error if any operation, such as initialization or
    ///   setting brightness, fails.
    ///
    /// ## Example
    /// ```rust
    /// display.init().expect("Failed to initialize the display");
    /// ```
    pub fn init(&mut self) -> Result<(), AppError> {
        self.tm1637.lock().unwrap().init()?;
        self.tm1637.lock().unwrap().set_brightness(5)?;

        self.write(DisplayMessage::Init.as_bytes())?;

        Ok(())
    }

    /// Writes a 4-byte message to the seven-segment display.
    ///
    /// This function locks the underlying `TM1637` driver, clears the display,
    /// and then writes the provided message in raw format starting from the
    /// first digit.
    ///
    /// ## Arguments
    /// - `message`: A fixed-size array of 4 bytes representing the digits or
    ///   characters to display.
    ///
    /// ## Returns
    /// - `Ok(())`: If the message is successfully written to the display.
    /// - `Err(AppError)`: An error if clearing or writing to the display fails.
    ///
    /// # Example
    /// ```rust
    /// display
    ///     .write(*b"1234")
    ///     .expect("Failed to write to the display");
    /// ```
    pub fn write(&mut self, message: [u8; 4]) -> Result<(), AppError> {
        let mut locked_display = self.tm1637.lock().unwrap();
        locked_display.clear()?;
        locked_display.print_raw(0, &message)?;

        Ok(())
    }

    /// Sets the brightness level of the seven-segment display.
    ///
    /// This function locks the underlying `TM1637` driver and updates the
    /// display brightness to the specified level.
    ///
    /// ## Arguments
    /// - `level`: The brightness level (typically from 0 to 7, depending on the
    ///   TM1637 driver).
    ///
    /// ## Returns
    /// - `Ok(())`: If the brightness is successfully updated.
    /// - `Err(AppError)`: An error if setting the brightness fails.
    ///
    /// ## Example
    /// ```rust
    /// display.set_brightness(5).expect("Failed to set brightness");
    /// ```
    pub fn set_brightness(&mut self, level: u8) -> Result<(), AppError> {
        self.tm1637.lock().unwrap().set_brightness(level)?;

        Ok(())
    }

    /// Updates the display to show the current time.
    ///
    /// This function retrieves the current time, converts the digits into the
    /// corresponding seven-segment display format, and writes them to the
    /// display. The colon separator (dot on the second digit) is enabled to
    /// represent time correctly.
    ///
    /// ## Returns
    /// - `Ok(())`: If the time is successfully retrieved and displayed.
    /// - `Err(AppError)`: An error if retrieving the time or updating the
    ///   display fails.
    ///
    /// ## Example
    /// ```rust
    /// display
    ///     .update_display_time()
    ///     .expect("Failed to update time on display");
    /// ```
    pub fn update_display_time(&mut self) -> Result<(), AppError> {
        let time = time::get_time();

        let digits = [
            DISPLAY_DIGIT[time[0] as usize],
            DISPLAY_DIGIT[time[1] as usize] | 0b10000000,
            DISPLAY_DIGIT[time[2] as usize],
            DISPLAY_DIGIT[time[3] as usize],
        ];

        self.write([digits[0], digits[1], digits[2], digits[3]])?;

        Ok(())
    }
}
