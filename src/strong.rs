
use embedded_hal::digital::ErrorType;

pub trait StrongPullupPin: ErrorType {
 
    /// Enable the strong pull-up on the pin. This is typically used to provide extra current to a device during a command, such as a temperature conversion.
    fn enable_strong_pullup(&mut self) -> Result<(), Self::Error>;

    /// Disable the strong pull-up on the pin.
    fn disable_strong_pullup(&mut self) -> Result<(), Self::Error>;
}