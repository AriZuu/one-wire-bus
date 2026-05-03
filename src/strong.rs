use embedded_hal::digital::{Error, ErrorType, InputPin, OutputPin};

pub trait StrongPullupPin: ErrorType {
    /// Enable the strong pull-up on the pin. This is typically used to provide extra current to a device during a command, such as a temperature conversion.
    fn enable_strong_pullup(&mut self) -> Result<(), Self::Error>;

    /// Disable the strong pull-up on the pin.
    fn disable_strong_pullup(&mut self) -> Result<(), Self::Error>;
}

pub struct DummyPullupPin<T> {
    pin: T,
}

impl<T, E> DummyPullupPin<T>
where
    T: InputPin<Error = E> + OutputPin<Error = E>,
{
    pub fn new(pin: T) -> Self {
        Self { pin }
    }
}

impl<T, E> ErrorType for DummyPullupPin<T>
where
    T: InputPin<Error = E> + OutputPin<Error = E>,
    E: Error,
{
    type Error = E;
}

impl<T, E> InputPin for DummyPullupPin<T>
where
    T: InputPin<Error = E> + OutputPin<Error = E>,
    E: Error,
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_low()
    }
}

impl<T, E> OutputPin for DummyPullupPin<T>
where
    T: InputPin<Error = E> + OutputPin<Error = E>,
    E: Error,
{
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}

impl<T, E> StrongPullupPin for DummyPullupPin<T>
where
    T: InputPin<Error = E> + OutputPin<Error = E>,
    E: Error,
{
    fn enable_strong_pullup(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn disable_strong_pullup(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
