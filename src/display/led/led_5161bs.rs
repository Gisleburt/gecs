use embedded_hal::digital::v2::OutputPin;

use crate::{characters, SevenSegmentDp};

/// Map the digital pins of your board to the pins of the display. If the display is face up, the
/// pins are numbered starting in the bottom left corner at one, going anti clockwise, ending on
/// pin 10 in the top left corner. Pins 3 and 8 in the middle are connected and are the high pins.
/// You do not need to connect this to a digital pin, but you may if you wish. The power to the
/// other pins is set Low to turn each segment on.
pub struct Display5161bs<P: OutputPin> {
    // E
    pub pin1: P,
    // D
    pub pin2: P,
    // C
    pub pin4: P,
    // DP
    pub pin5: P,
    // B
    pub pin6: P,
    // A
    pub pin7: P,
    // F
    pub pin9: P,
    // G
    pub pin10: P,
    // Ground
    //
    // Note: There are two power (high) pins, 3 and 8. You only need to connect one of them as they
    // are connected to each other.
    pub power: Option<P>,
}

impl<P: OutputPin> Display5161bs<P> {
    pub fn off(&mut self) -> Result<(), P::Error> {
        self.display_digit(characters::CHAR_OFF)
    }

    pub fn display_char(&mut self, c: &char) -> Result<(), P::Error> {
        self.display_digit(c.into())
    }

    pub fn display_digit(&mut self, c: SevenSegmentDp) -> Result<(), P::Error> {
        // Ground
        if let Some(power) = self.power.as_mut() {
            power.set_high()?
        }
        // E
        c.e.apply_to_pin(&mut self.pin1)?;
        // D
        c.d.apply_to_pin(&mut self.pin2)?;
        // C
        c.c.apply_to_pin(&mut self.pin4)?;
        // DP
        c.dp.apply_to_pin(&mut self.pin5)?;
        // B
        c.b.apply_to_pin(&mut self.pin6)?;
        // A
        c.a.apply_to_pin(&mut self.pin7)?;
        // F
        c.f.apply_to_pin(&mut self.pin9)?;
        // G
        c.g.apply_to_pin(&mut self.pin10)?;

        Ok(())
    }
}
