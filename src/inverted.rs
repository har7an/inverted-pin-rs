use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

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

impl<P, E> OutputPin for InvertedPin<P>
where
    P: OutputPin<Error = E>,
{
    type Error = E;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}

impl<P, E> InputPin for InvertedPin<P>
where
    P: InputPin<Error = E>,
{
    type Error = E;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.pin.is_low()
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }
}

impl<P, E> ToggleableOutputPin for InvertedPin<P>
where
    P: ToggleableOutputPin<Error = E>,
{
    type Error = E;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}

impl<P, E> StatefulOutputPin for InvertedPin<P>
where
    P: StatefulOutputPin<Error = E>,
{
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        self.pin.is_set_low()
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        self.pin.is_set_high()
    }
}
