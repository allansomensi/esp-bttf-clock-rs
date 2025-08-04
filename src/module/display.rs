use super::led::SharedAmPmIndicator;
use crate::{
    error::AppError,
    service::{display::SevenSegmentDisplayService, led::AmPmIndicatorService},
    time,
    util::{messages::DisplayMessage, DISPLAY_DIGIT},
};
use esp_idf_svc::hal::{
    delay::Ets,
    gpio::{IOPin, InputOutput, Output, OutputPin, PinDriver},
};
use std::sync::{Arc, Mutex};
use tm1637::TM1637;

/// A thread-safe shared [`SevenSegmentDisplay`] using `Arc<Mutex<...>>`.
pub type SharedSevenSegmentDisplay<'a, CLK, DIO> = Arc<Mutex<SevenSegmentDisplay<'a, CLK, DIO>>>;

/// Centralizes the logic for controlling a seven-segment display.
pub struct SevenSegmentDisplay<'a, CLK: OutputPin, DIO: IOPin> {
    tm1637: TM1637<'a, PinDriver<'a, CLK, Output>, PinDriver<'a, DIO, InputOutput>, Ets>,
}

/// Groups together the shared instances of the seven-segment displays.
pub struct DisplayGroup<'a, CLK: OutputPin, DateDIO: IOPin, YearDIO: IOPin, HourDIO: IOPin> {
    pub date: SharedSevenSegmentDisplay<'a, CLK, DateDIO>,
    pub year: SharedSevenSegmentDisplay<'a, CLK, YearDIO>,
    pub hour: SharedSevenSegmentDisplay<'a, CLK, HourDIO>,
}

/// A type alias for a thread-safe, shared group of seven-segment displays.
pub type SharedDisplayGroup<'a, CLK, DateDIO, YearDIO, HourDIO> =
    Arc<Mutex<DisplayGroup<'static, CLK, DateDIO, YearDIO, HourDIO>>>;

impl<CLK, DIO> SevenSegmentDisplay<'_, CLK, DIO>
where
    CLK: OutputPin,
    DIO: IOPin,
{
    /// Creates a new [`SevenSegmentDisplay`] instance.
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
    pub fn new<'a>(
        clk: CLK,
        dio: DIO,
    ) -> Result<SharedSevenSegmentDisplay<'a, CLK, DIO>, AppError> {
        let clk = Box::new(PinDriver::output(clk)?);
        let dio = Box::new(PinDriver::input_output(dio)?);
        let delay = Box::new(Ets);

        let tm1637 = TM1637::new(Box::leak(clk), Box::leak(dio), Box::leak(delay));
        let display = SevenSegmentDisplay { tm1637 };

        Ok(SharedSevenSegmentDisplay::new(display.into()))
    }
}

impl<CLK, DIO> SevenSegmentDisplayService for SevenSegmentDisplay<'_, CLK, DIO>
where
    CLK: OutputPin,
    DIO: IOPin,
{
    /// Initializes the [SevenSegmentDisplay] by setting up the display and
    /// configuring the brightness.
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
    fn init(&mut self) -> Result<(), AppError> {
        self.tm1637.init()?;
        self.tm1637.set_brightness(0)?;

        self.write(DisplayMessage::Init.as_bytes())?;

        Ok(())
    }

    /// Writes a 4-byte message to the seven-segment display.
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
    fn write(&mut self, message: [u8; 4]) -> Result<(), AppError> {
        self.tm1637.clear()?;
        self.tm1637.print_raw(0, &message)?;

        Ok(())
    }

    /// Sets the brightness level of the seven-segment display.
    ///
    /// ## Arguments
    /// - `level`: The brightness level (0-7).
    ///
    /// ## Returns
    /// - `Ok(())`: If the brightness is successfully updated.
    /// - `Err(AppError)`: An error if setting the brightness fails.
    ///
    /// ## Example
    /// ```rust
    /// display.set_brightness(5).expect("Failed to set brightness");
    /// ```
    fn set_brightness(&mut self, level: u8) -> Result<(), AppError> {
        self.tm1637.set_brightness(level)?;

        Ok(())
    }

    /// Updates the display to show the current time.
    ///
    /// ## Returns
    /// - `Ok(())`: If the time is successfully retrieved and displayed.
    /// - `Err(AppError)`: An error if retrieving the time or updating the
    ///   display fails.
    ///
    /// ## Example
    /// ```rust
    /// display
    ///     .update_display_time(am_pm_indicator)
    ///     .expect("Failed to update time on display");
    /// ```
    fn update_display_hour<AM: OutputPin, PM: OutputPin>(
        &mut self,
        am_pm_indicator: SharedAmPmIndicator<AM, PM>,
    ) -> Result<(), AppError> {
        let time = time::get_hour_min();

        let digits = [
            DISPLAY_DIGIT[time[0] as usize],
            DISPLAY_DIGIT[time[1] as usize] | 0b10000000,
            DISPLAY_DIGIT[time[2] as usize],
            DISPLAY_DIGIT[time[3] as usize],
        ];

        self.write([digits[0], digits[1], digits[2], digits[3]])?;

        let hour = time[0] * 10 + time[1];

        if hour < 12 {
            am_pm_indicator.lock().unwrap().set_am()?;
        } else {
            am_pm_indicator.lock().unwrap().set_pm()?;
        }

        Ok(())
    }

    /// Updates the display to show the current year.
    ///
    /// ## Returns
    /// - `Ok(())`: If the year is successfully retrieved and displayed.
    /// - `Err(AppError)`: An error if retrieving the year or updating the
    ///   display fails.
    ///
    /// ## Example
    /// ```rust
    /// display
    ///     .update_display_year()
    ///     .expect("Failed to update year on display");
    /// ```
    fn update_display_year(&mut self) -> Result<(), AppError> {
        let year = time::get_year();

        let digits = [
            DISPLAY_DIGIT[year[0] as usize],
            DISPLAY_DIGIT[year[1] as usize],
            DISPLAY_DIGIT[year[2] as usize],
            DISPLAY_DIGIT[year[3] as usize],
        ];

        self.write(digits)?;

        Ok(())
    }

    /// Updates the display to show the current date.
    ///
    /// ## Returns
    /// - `Ok(())`: If the date is successfully retrieved and displayed.
    /// - `Err(AppError)`: An error if retrieving the date or updating the
    ///   display fails.
    ///
    /// ## Example
    /// ```rust
    /// display
    ///     .update_display_date()
    ///     .expect("Failed to update date on display");
    /// ```
    fn update_display_date(&mut self) -> Result<(), AppError> {
        let (day, month) = time::get_day_month();

        let digits = [
            DISPLAY_DIGIT[(day / 10) as usize],
            DISPLAY_DIGIT[(day % 10) as usize] | 0b10000000,
            DISPLAY_DIGIT[(month / 10) as usize],
            DISPLAY_DIGIT[(month % 10) as usize],
        ];

        self.write(digits)?;

        Ok(())
    }
}
