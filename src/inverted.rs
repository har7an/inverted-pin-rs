use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

/// Inverted output pin
///
/// Whenever setting this pin to a high or low level, the wrapped pin will be set to the
/// opposite level.
#[derive(Debug, Clone, Copy)]
pub struct InvertedOutputPin<P> {
    pin: P,
}

impl<P, E> InvertedOutputPin<P>
where
    P: OutputPin<Error = E>,
{
    /// Create new instance
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    /// Destroy instance and return the wrapped pin
    pub fn destroy(self) -> P {
        self.pin
    }
}

impl<P, E> OutputPin for InvertedOutputPin<P>
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

/// Inverted input pin
///
/// Whenever reading this pin it will read the wrapped input pin and return
/// the opposite level.
#[derive(Debug, Clone, Copy)]
pub struct InvertedInputPin<P> {
    pin: P,
}

impl<P, E> InvertedInputPin<P>
where
    P: InputPin<Error = E>,
{
    /// Create new instance
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    /// Destroy instance and return the wrapped pin
    pub fn destroy(self) -> P {
        self.pin
    }
}

impl<P, E> InputPin for InvertedInputPin<P>
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

impl<P, E> ToggleableOutputPin for InvertedOutputPin<P>
where
    P: ToggleableOutputPin<Error = E>,
{
    type Error = E;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}

impl<P, E> StatefulOutputPin for InvertedOutputPin<P>
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
