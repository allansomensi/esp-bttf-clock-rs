use crate::{error::AppError, service::led::AmPmIndicatorService};
use esp_idf_svc::hal::{
    gpio::{Output, OutputPin, PinDriver},
    peripheral::Peripheral,
};
use std::sync::{Arc, Mutex};

pub type SharedAmPmIndicator<'a, AM, PM> = Arc<Mutex<AmPmIndicator<'a, AM, PM>>>;

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
    fn set_am(&mut self) -> Result<(), AppError> {
        self.am.lock().unwrap().set_high()?;
        self.pm.lock().unwrap().set_low()?;

        Ok(())
    }

    fn set_pm(&mut self) -> Result<(), AppError> {
        self.am.lock().unwrap().set_low()?;
        self.pm.lock().unwrap().set_high()?;

        Ok(())
    }

    fn clear(&mut self) -> Result<(), AppError> {
        self.am.lock().unwrap().set_low()?;
        self.pm.lock().unwrap().set_low()?;

        Ok(())
    }
}
