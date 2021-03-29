#![no_std]

// use embedded_hal::digital::{OutputPin, PinState::{High, Low}, PinState}; // ToDo: Use this for v1
// use core::ops::Not; // ToDo: Use this for v1

// v0.2 hack
use embedded_hal::digital::v2::OutputPin;
enum PinState {
    High,
    Low,
}
use PinState::{High, Low};

/// Map the digital pins of your board to the pins of the display. If the display is face up, the
/// pins are numbered starting in the bottom left corner at one, going anti clockwise, ending on
/// pin 10 in the top left corner. Pins 3 and 8 in the middle are connected and are the high pins.
/// You do not need to connect this to a digital pin, but you may if you wish. The power to the
/// other pins is set Low to turn each segment on.
pub struct LedDisplay5161bs<P: OutputPin> {
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

pub struct Digit {
    a: PinState,
    b: PinState,
    c: PinState,
    d: PinState,
    e: PinState,
    f: PinState,
    g: PinState,
    dp: PinState,
}

// ToDo: Use in v1
// impl Not for Digit {
//     type Output = Digit;
//
//     fn not(self) -> Self::Output {
//         Digit {
//             a: !self.a,
//             b: !self.b,
//             c: !self.c,
//             d: !self.d,
//             e: !self.e,
//             f: !self.f,
//             g: !self.g,
//             dp: !self.dp,
//         }
//     }
// }

const CHAR_OFF: Digit = Digit {
    a: High,
    b: High,
    c: High,
    d: High,
    e: High,
    f: High,
    g: High,
    dp: High,
};
const CHAR_0: Digit = Digit {
    a: Low,
    b: Low,
    c: Low,
    d: Low,
    e: Low,
    f: Low,
    g: High,
    dp: High,
};
const CHAR_1: Digit = Digit {
    a: High,
    b: Low,
    c: Low,
    d: High,
    e: High,
    f: High,
    g: High,
    dp: High,
};
const CHAR_2: Digit = Digit {
    a: Low,
    b: Low,
    c: High,
    d: Low,
    e: Low,
    f: High,
    g: Low,
    dp: High,
};
const CHAR_3: Digit = Digit {
    a: Low,
    b: Low,
    c: Low,
    d: Low,
    e: High,
    f: High,
    g: Low,
    dp: High,
};
const CHAR_4: Digit = Digit {
    a: High,
    b: Low,
    c: Low,
    d: High,
    e: High,
    f: Low,
    g: Low,
    dp: High,
};
const CHAR_5: Digit = Digit {
    a: Low,
    b: High,
    c: Low,
    d: Low,
    e: High,
    f: Low,
    g: Low,
    dp: High,
};
const CHAR_6: Digit = Digit {
    a: Low,
    b: High,
    c: Low,
    d: Low,
    e: Low,
    f: Low,
    g: Low,
    dp: High,
};
const CHAR_7: Digit = Digit {
    a: Low,
    b: Low,
    c: Low,
    d: High,
    e: High,
    f: High,
    g: High,
    dp: High,
};
const CHAR_8: Digit = Digit {
    a: Low,
    b: Low,
    c: Low,
    d: Low,
    e: Low,
    f: Low,
    g: Low,
    dp: High,
};
const CHAR_9: Digit = Digit {
    a: Low,
    b: Low,
    c: Low,
    d: Low,
    e: High,
    f: Low,
    g: Low,
    dp: High,
};
const CHAR_DP: Digit = Digit {
    a: High,
    b: High,
    c: High,
    d: High,
    e: High,
    f: High,
    g: High,
    dp: Low,
};
const CHAR_MINUS: Digit = Digit {
    a: High,
    b: High,
    c: High,
    d: High,
    e: High,
    f: High,
    g: Low,
    dp: High,
};
const CHAR_A: Digit = Digit {
    a: Low,
    b: Low,
    c: Low,
    d: High,
    e: Low,
    f: Low,
    g: Low,
    dp: Low,
};
const CHAR_B: Digit = Digit {
    a: Low,
    b: Low,
    c: Low,
    d: Low,
    e: Low,
    f: Low,
    g: Low,
    dp: Low,
};
const CHAR_C: Digit = Digit {
    a: Low,
    b: High,
    c: High,
    d: Low,
    e: Low,
    f: Low,
    g: High,
    dp: Low,
};
const CHAR_D: Digit = Digit {
    a: Low,
    b: Low,
    c: Low,
    d: Low,
    e: Low,
    f: Low,
    g: High,
    dp: Low,
};
const CHAR_E: Digit = Digit {
    a: Low,
    b: High,
    c: High,
    d: Low,
    e: Low,
    f: Low,
    g: Low,
    dp: Low,
};
const CHAR_F: Digit = Digit {
    a: Low,
    b: High,
    c: High,
    d: High,
    e: Low,
    f: Low,
    g: Low,
    dp: Low,
};
const CHAR_ERROR: Digit = Digit {
    a: Low,
    b: Low,
    c: High,
    d: Low,
    e: Low,
    f: Low,
    g: Low,
    dp: Low,
};

impl From<&char> for Digit {
    fn from(c: &char) -> Self {
        match c {
            '0' => CHAR_0,
            '1' => CHAR_1,
            '2' => CHAR_2,
            '3' => CHAR_3,
            '4' => CHAR_4,
            '5' => CHAR_5,
            '6' => CHAR_6,
            '7' => CHAR_7,
            '8' => CHAR_8,
            '9' => CHAR_9,
            '.' => CHAR_DP,
            '-' => CHAR_MINUS,
            'A' | 'a' => CHAR_A,
            'B' | 'b' => CHAR_B,
            'C' | 'c' => CHAR_C,
            'D' | 'd' => CHAR_D,
            'E' | 'e' => CHAR_E,
            'F' | 'f' => CHAR_F,
            _ => CHAR_ERROR,
        }
    }
}

impl<P: OutputPin> LedDisplay5161bs<P> {
    // v0.2 hack
    fn set_state(pin: &mut P, state: PinState) -> Result<(), P::Error> {
        match state {
            High => pin.set_high()?,
            Low => pin.set_low()?,
        };
        Ok(())
    }

    pub fn off(&mut self) -> Result<(), P::Error> {
        self.display_digit(CHAR_OFF)
    }

    pub fn display_char(&mut self, c: &char) -> Result<(), P::Error> {
        self.display_digit(c.into())
    }

    pub fn display_digit(&mut self, c: Digit) -> Result<(), P::Error> {
        // Ground
        if let Some(power) = self.power.as_mut() {
            power.set_high()?
        }
        // E
        LedDisplay5161bs::<P>::set_state(&mut self.pin1, c.e)?;
        // D
        LedDisplay5161bs::<P>::set_state(&mut self.pin2, c.d)?;
        // C
        LedDisplay5161bs::<P>::set_state(&mut self.pin4, c.c)?;
        // DP
        LedDisplay5161bs::<P>::set_state(&mut self.pin5, c.dp)?;
        // B
        LedDisplay5161bs::<P>::set_state(&mut self.pin6, c.b)?;
        // A
        LedDisplay5161bs::<P>::set_state(&mut self.pin7, c.a)?;
        // F
        LedDisplay5161bs::<P>::set_state(&mut self.pin9, c.f)?;
        // G
        LedDisplay5161bs::<P>::set_state(&mut self.pin10, c.g)?;

        Ok(())
    }
}
