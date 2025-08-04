use crate::{error::AppError, service::led::AmPmIndicatorService};
use esp_idf_svc::hal::{
    gpio::{Output, OutputPin, PinDriver},
    peripheral::Peripheral,
};
use std::sync::{Arc, Mutex};

/// A type alias for a thread-safe, shared instance of [`AmPmIndicator`].
pub type SharedAmPmIndicator<'a, AM, PM> = Arc<Mutex<AmPmIndicator<'a, AM, PM>>>;

/// Manages two output pins to act as AM and PM indicator LEDs.
pub struct AmPmIndicator<'a, AM, PM>
where
    AM: OutputPin,
    PM: OutputPin,
{
    am: Arc<Mutex<PinDriver<'a, AM, Output>>>,
    pm: Arc<Mutex<PinDriver<'a, PM, Output>>>,
}

impl<'a, AM, PM> AmPmIndicator<'a, AM, PM>
where
    AM: Peripheral<P = AM> + OutputPin + 'a,
    PM: Peripheral<P = PM> + OutputPin + 'a,
{
    /// Creates a new [`AmPmIndicator`] instance.
    ///
    /// Initializes the given pins as output drivers for the AM and PM LEDs.
    ///
    /// ## Arguments
    /// - `am_pin`: The GPIO pin designated for the AM indicator.
    /// - `pm_pin`: The GPIO pin designated for the PM indicator.
    ///
    /// ## Returns
    /// A `Result` containing a [`SharedAmPmIndicator`] on success, or an
    /// `AppError` if pin setup fails.
    ///
    /// ## Example
    /// ```rust
    /// let am_pm_indicator_shared =
    ///     AmPmIndicator::new(am_gpio_pin, pm_gpio_pin).expect("Failed to create AM/PM indicator");
    /// ```
    pub fn new(am_pin: AM, pm_pin: PM) -> Result<SharedAmPmIndicator<'a, AM, PM>, AppError> {
        let am = Arc::new(Mutex::new(PinDriver::output(am_pin)?));
        let pm = Arc::new(Mutex::new(PinDriver::output(pm_pin)?));

        let am_pm_indicator = Self { am, pm };

        Ok(SharedAmPmIndicator::new(am_pm_indicator.into()))
    }
}

impl<'a, AM, PM> AmPmIndicatorService for AmPmIndicator<'a, AM, PM>
where
    AM: Peripheral<P = AM> + OutputPin + 'a,
    PM: Peripheral<P = PM> + OutputPin + 'a,
{
    /// Activates the AM indicator LED and deactivates the PM LED.
    fn set_am(&mut self) -> Result<(), AppError> {
        self.am.lock().unwrap().set_high()?;
        self.pm.lock().unwrap().set_low()?;

        Ok(())
    }

    /// Activates the PM indicator LED and deactivates the AM LED.
    fn set_pm(&mut self) -> Result<(), AppError> {
        self.am.lock().unwrap().set_low()?;
        self.pm.lock().unwrap().set_high()?;

        Ok(())
    }

    /// Deactivates both the AM and PM indicator LEDs.
    fn clear(&mut self) -> Result<(), AppError> {
        self.am.lock().unwrap().set_low()?;
        self.pm.lock().unwrap().set_low()?;

        Ok(())
    }
}
