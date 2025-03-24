use std::sync::{Arc, Mutex};

/// Represents a request to set or retrieve a timezone.
///
/// This struct is used to deserialize incoming requests containing a timezone
/// value as a string. It is typically used to handle requests to set or update
/// the timezone of a system or device.
///
/// ## Fields
/// * `timezone` - A string representing the desired timezone, e.g.,
///   `"America/New_York"`.
///
/// ## Example
/// ```rust
/// let timezone_request: TimezoneRequest =
///     serde_json::from_str("{\"timezone\":\"America/New_York\"}").unwrap();
/// ```
#[derive(Clone, serde::Deserialize)]
pub struct TimezoneRequest {
    pub timezone: String,
}

lazy_static::lazy_static! {
    pub static ref TIMEZONE: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

/// Retrieves the current timezone, either from the global [TIMEZONE] or the
/// default environment value.
///
/// This function checks if a timezone has been set in the global [TIMEZONE]
/// variable. If a timezone is set, it returns that timezone as a string.
/// Otherwise, it returns the default timezone defined by the `DEFAULT_TIMEZONE`
/// environment variable.
///
/// ## Returns
/// A string representing the current timezone.
///
/// ## Example
/// ```rust
/// let timezone = get_timezone();
/// ```
pub fn get_timezone() -> String {
    let timezone = TIMEZONE.lock().unwrap();
    match &*timezone {
        Some(tz) => tz.clone(),
        None => env!("DEFAULT_TIMEZONE").to_string(),
    }
}

/// Sets the global timezone to the provided value.
///
/// This function updates the global [TIMEZONE] variable with the new timezone
/// value.
///
/// ## Arguments
/// * `new_timezone` - A string representing the new timezone.
///
/// ## Example
/// ```rust
/// set_timezone("America/New_York".to_string());
/// ```
pub fn set_timezone(new_timezone: String) {
    let mut timezone = TIMEZONE.lock().unwrap();
    *timezone = Some(new_timezone);
}
