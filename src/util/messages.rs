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
