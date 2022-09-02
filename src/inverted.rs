use embedded_hal::digital::blocking::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};
use embedded_hal::digital::ErrorType;

/// Inverted input/output pin
///
/// If wrapping an output pin, whenever setting this pin to a high or low level,
/// the wrapped pin will be set to the opposite level.
///
/// Likewise, if wrapping an input pin, whenever reading this pin it will read
/// the wrapped input pin and return the opposite level.
#[derive(Debug, Clone, Copy)]
pub struct InvertedPin<P> {
    pin: P,
}

impl<P> InvertedPin<P> {
    /// Create new instance
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    /// Destroy instance and return the wrapped pin
    pub fn destroy(self) -> P {
        self.pin
    }
}

impl<P> ErrorType for InvertedPin<P>
where
    P: ErrorType
{
    type Error = P::Error;
}

impl<P> OutputPin for InvertedPin<P>
where
    P: OutputPin,
{
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}

impl<P> InputPin for InvertedPin<P>
where
    P: InputPin,
{
    fn is_high(&self) -> Result<bool, Self::Error> {
        self.pin.is_low()
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }
}

impl<P> ToggleableOutputPin for InvertedPin<P>
where
    P: ToggleableOutputPin,
{
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}

impl<P> StatefulOutputPin for InvertedPin<P>
where
    P: StatefulOutputPin,
{
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        self.pin.is_set_low()
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        self.pin.is_set_high()
    }
}
