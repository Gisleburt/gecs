use core::ops::Not;
use embedded_hal::digital::v2::OutputPin;

/// In embedded-hal 1.0 there will be a digital pin state that we should switch to once its
/// stabilised, in the meantime, this makes a good stand in.
pub enum DigitalPinState {
    High,
    Low,
}

use DigitalPinState::{High, Low};

impl DigitalPinState {
    pub fn apply_to_pin<P: OutputPin>(self, pin: &mut P) -> Result<(), P::Error> {
        match self {
            High => pin.set_high()?,
            Low => pin.set_low()?,
        };
        Ok(())
    }
}

impl Not for DigitalPinState {
    type Output = DigitalPinState;

    fn not(self) -> Self::Output {
        match self {
            High => Low,
            Low => High,
        }
    }
}
