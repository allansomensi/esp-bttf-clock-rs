use crate::{error::AppError, time, util::DISPLAY_DIGIT};
use esp_idf_svc::hal::{
    delay::Ets,
    gpio::{IOPin, InputOutput, Output, OutputPin, PinDriver},
};
use std::sync::{Arc, Mutex};
use tm1637::TM1637;

/// Type alias for the [TM1637] display using pin drivers and [Ets] as time control.
/// This is an ´Arc<Mutex<>>´ to ensure thread safety and shared access to the display.
pub type SharedTm1637<CLK, DIO> = Arc<
    Mutex<
        TM1637<'static, PinDriver<'static, CLK, Output>, PinDriver<'static, DIO, InputOutput>, Ets>,
    >,
>;

/// Enum representing different display messages.
/// Used to send specific byte patterns to the display.
pub enum DisplayMessage {
    Init,
    Sync,
}

impl DisplayMessage {
    /// Converts the display message into a 4-byte array that represents the bits to be shown on the display.
    ///
    /// ## Returns
    /// Returns a 4-byte array, each byte representing a value for display output.
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

/// Creates a [TM1637] display object.
///
/// ## Arguments
/// - `clk`: Pin for the clock of the display (implements [OutputPin]).
/// - `dio`: Pin for the data of the display (implements [IOPin]).
///
/// ## Returns
/// The result is an instance of the [TM1637] ready for interaction.
///
/// ## Example
/// ```
/// let display = get_display(clk_pin, dio_pin).expect("Failed to create display");
/// ```
pub fn get_display<CLK, DIO>(clk: CLK, dio: DIO) -> Result<SharedTm1637<CLK, DIO>, AppError>
where
    CLK: OutputPin,
    DIO: IOPin,
{
    let clk = Box::new(PinDriver::output(clk)?);
    let dio = Box::new(PinDriver::input_output(dio)?);
    let delay = Box::new(Ets);

    let display = TM1637::new(Box::leak(clk), Box::leak(dio), Box::leak(delay));

    Ok(Arc::new(Mutex::new(display)))
}

/// Initializes the display with an initial message and sets its brightness.
///
/// ## Arguments
/// - `display`: A reference to [SharedTm1637].
///
/// ## Returns
/// `Ok(())` if the initialization is successful, or an [AppError] if it fails.
///
/// ## Example
/// ```rust
/// init_display(&display).expect("Failed to initialize display");
/// ```
pub fn init_display<CLK, DIO>(display: &SharedTm1637<CLK, DIO>) -> Result<(), AppError>
where
    CLK: OutputPin,
    DIO: IOPin,
{
    let init_message = DisplayMessage::Init.as_bytes();

    display.lock().unwrap().init()?;
    display.lock().unwrap().set_brightness(5)?;
    write(display, init_message)?;

    Ok(())
}

/// Updates the display with the current time formatted into digits.
/// The time is fetched using [time::get_time] and displayed in a 4-digit format.
///
/// ## Arguments
/// - `display`: A reference to [SharedTm1637].
///
/// ## Returns
/// `Ok(())` if the time update is successful, or an [AppError] if it fails.
///
/// ## Example
/// ```rust
/// update_display_time(&display).expect("Failed to update display time");
/// ```
pub fn update_display_time<CLK, DIO>(display: &SharedTm1637<CLK, DIO>) -> Result<(), AppError>
where
    CLK: OutputPin,
    DIO: IOPin,
{
    let time = time::get_time();

    let digits = [
        DISPLAY_DIGIT[time[0] as usize],
        DISPLAY_DIGIT[time[1] as usize] | 0b10000000,
        DISPLAY_DIGIT[time[2] as usize],
        DISPLAY_DIGIT[time[3] as usize],
    ];

    write(display, [digits[0], digits[1], digits[2], digits[3]])?;

    Ok(())
}

/// Writes a 4-byte message to the display, clearing it first and then printing the message.
///
/// ## Arguments
/// - `display`: A reference to [SharedTm1637].
/// - `message`: A 4-byte array representing the message to be displayed on the screen. Each byte corresponds to a digit shown.
///
/// ## Returns
/// Returns `Ok(())` if the message is successfully written to the display, or an [AppError] if any error occurs during the process.
///
/// ## Example
/// ```rust
/// write(&display, [0b01100001, 0b01100010, 0b01100011, 0b01100100])
///     .expect("Error printing message");
/// ```
pub fn write<CLK, DIO>(display: &SharedTm1637<CLK, DIO>, message: [u8; 4]) -> Result<(), AppError>
where
    CLK: OutputPin,
    DIO: IOPin,
{
    let mut locked_display = display.lock().unwrap();
    locked_display.clear()?;
    locked_display.print_raw(0, &message)?;

    Ok(())
}
