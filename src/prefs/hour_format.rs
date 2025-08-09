use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    /// A global, thread-safe static variable to hold the current hour format setting.
    pub static ref HOUR_FORMAT: Arc<Mutex<Option<HourFormat>>> = Arc::new(Mutex::new(None));
}

/// Represents the hour format setting, either 12-hour or 24-hour.
#[derive(Default, Copy, Clone)]
pub enum HourFormat {
    Twelve = 0,
    #[default]
    TwentyFour = 1,
}

/// Allows converting a u8 integer into an [`HourFormat`] enum.
impl From<u8> for HourFormat {
    fn from(value: u8) -> Self {
        match value {
            0 => HourFormat::Twelve,
            1 => HourFormat::TwentyFour,
            _ => HourFormat::default(),
        }
    }
}

/// Retrieves the current global hour format setting in a thread-safe way.
pub fn get_hour_format() -> HourFormat {
    let hour_format_guard = HOUR_FORMAT.lock().unwrap();

    match &*hour_format_guard {
        Some(hour_format) => *hour_format,
        None => HourFormat::default(),
    }
}

/// Updates the global hour format setting in a thread-safe way.
pub fn set_hour_format(new_hour_format: HourFormat) {
    let mut hour_format_guard = HOUR_FORMAT.lock().unwrap();
    *hour_format_guard = Some(new_hour_format);
}
