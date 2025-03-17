/// A lookup table for displaying digits (0-9) on a 7-segment display.
pub const DISPLAY_DIGIT: [u8; 10] = [
    0b00111111, // 0
    0b00000110, // 1
    0b01011011, // 2
    0b01001111, // 3
    0b01100110, // 4
    0b01101101, // 5
    0b01111101, // 6
    0b00000111, // 7
    0b01111111, // 8
    0b01101111, // 9
];

/// Extracts the digits from the URL query string after the "?" symbol.
///
/// This function finds the query string in the URL, extracts the part after the "?" symbol,
/// and then returns the digits found before the next "&" symbol, if present.
///
/// # Parameters
/// - `url`: A string slice representing the URL to be processed.
///
/// # Returns
/// A vector of bytes representing the extracted digits from the query string, or an empty vector if no digits are found.
///
/// # Example
/// ```rust
/// let digits = find_digits_in_url("http://example.com?12345&other_param=value");
/// assert_eq!(digits, b"12345");
/// ```
///
pub fn find_digits_in_url(url: &str) -> Vec<u8> {
    // Find the position of the '?' character in the URL
    if let Some(start) = url.find('?') {
        let digits_value = &url[start + 1..];
        // Find the position of the '&' character to determine where the digits end
        let end = digits_value.find('&').unwrap_or(digits_value.len());
        let digits = &digits_value[..end];

        return digits.as_bytes().to_vec();
    }

    // If `?`` is not found, return an empty vector
    Vec::new()
}
