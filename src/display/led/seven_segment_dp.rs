// use embedded_hal::digital::{OutputPin, PinState::{High, Low}, PinState}; // ToDo: Use this for v1
// use core::ops::Not; // ToDo: Use this for v1

use core::convert::From;
use core::ops::Not;
use crate::DigitalPinState::{self, High, Low};

pub struct SevenSegmentDp {
    pub a: DigitalPinState,
    pub b: DigitalPinState,
    pub c: DigitalPinState,
    pub d: DigitalPinState,
    pub e: DigitalPinState,
    pub f: DigitalPinState,
    pub g: DigitalPinState,
    pub dp: DigitalPinState,
}

impl Not for SevenSegmentDp {
    type Output = SevenSegmentDp;

    fn not(self) -> Self::Output {
        SevenSegmentDp {
            a: !self.a,
            b: !self.b,
            c: !self.c,
            d: !self.d,
            e: !self.e,
            f: !self.f,
            g: !self.g,
            dp: !self.dp,
        }
    }
}

pub mod characters {
    use super::*;

    pub const CHAR_OFF: SevenSegmentDp = SevenSegmentDp {
        a: High,
        b: High,
        c: High,
        d: High,
        e: High,
        f: High,
        g: High,
        dp: High,
    };

    pub const CHAR_0: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: Low,
        c: Low,
        d: Low,
        e: Low,
        f: Low,
        g: High,
        dp: High,
    };

    pub const CHAR_1: SevenSegmentDp = SevenSegmentDp {
        a: High,
        b: Low,
        c: Low,
        d: High,
        e: High,
        f: High,
        g: High,
        dp: High,
    };
    pub const CHAR_2: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: Low,
        c: High,
        d: Low,
        e: Low,
        f: High,
        g: Low,
        dp: High,
    };
    pub const CHAR_3: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: Low,
        c: Low,
        d: Low,
        e: High,
        f: High,
        g: Low,
        dp: High,
    };

    pub const CHAR_4: SevenSegmentDp = SevenSegmentDp {
        a: High,
        b: Low,
        c: Low,
        d: High,
        e: High,
        f: Low,
        g: Low,
        dp: High,
    };

    pub const CHAR_5: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: High,
        c: Low,
        d: Low,
        e: High,
        f: Low,
        g: Low,
        dp: High,
    };

    pub const CHAR_6: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: High,
        c: Low,
        d: Low,
        e: Low,
        f: Low,
        g: Low,
        dp: High,
    };

    pub const CHAR_7: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: Low,
        c: Low,
        d: High,
        e: High,
        f: High,
        g: High,
        dp: High,
    };

    pub const CHAR_8: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: Low,
        c: Low,
        d: Low,
        e: Low,
        f: Low,
        g: Low,
        dp: High,
    };

    pub const CHAR_9: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: Low,
        c: Low,
        d: Low,
        e: High,
        f: Low,
        g: Low,
        dp: High,
    };

    pub const CHAR_DP: SevenSegmentDp = SevenSegmentDp {
        a: High,
        b: High,
        c: High,
        d: High,
        e: High,
        f: High,
        g: High,
        dp: Low,
    };

    pub const CHAR_MINUS: SevenSegmentDp = SevenSegmentDp {
        a: High,
        b: High,
        c: High,
        d: High,
        e: High,
        f: High,
        g: Low,
        dp: High,
    };

    pub const CHAR_A: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: Low,
        c: Low,
        d: High,
        e: Low,
        f: Low,
        g: Low,
        dp: Low,
    };

    pub const CHAR_B: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: Low,
        c: Low,
        d: Low,
        e: Low,
        f: Low,
        g: Low,
        dp: Low,
    };

    pub const CHAR_C: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: High,
        c: High,
        d: Low,
        e: Low,
        f: Low,
        g: High,
        dp: Low,
    };

    pub const CHAR_D: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: Low,
        c: Low,
        d: Low,
        e: Low,
        f: Low,
        g: High,
        dp: Low,
    };

    pub const CHAR_E: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: High,
        c: High,
        d: Low,
        e: Low,
        f: Low,
        g: Low,
        dp: Low,
    };

    pub const CHAR_F: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: High,
        c: High,
        d: High,
        e: Low,
        f: Low,
        g: Low,
        dp: Low,
    };

    pub const CHAR_ERROR: SevenSegmentDp = SevenSegmentDp {
        a: Low,
        b: Low,
        c: High,
        d: Low,
        e: Low,
        f: Low,
        g: Low,
        dp: Low,
    };
}


impl From<&char> for SevenSegmentDp {
    fn from(c: &char) -> Self {
        match c {
            '0' => characters::CHAR_0,
            '1' => characters::CHAR_1,
            '2' => characters::CHAR_2,
            '3' => characters::CHAR_3,
            '4' => characters::CHAR_4,
            '5' => characters::CHAR_5,
            '6' => characters::CHAR_6,
            '7' => characters::CHAR_7,
            '8' => characters::CHAR_8,
            '9' => characters::CHAR_9,
            '.' => characters::CHAR_DP,
            '-' => characters::CHAR_MINUS,
            'A' | 'a' => characters::CHAR_A,
            'B' | 'b' => characters::CHAR_B,
            'C' | 'c' => characters::CHAR_C,
            'D' | 'd' => characters::CHAR_D,
            'E' | 'e' => characters::CHAR_E,
            'F' | 'f' => characters::CHAR_F,
            _ => characters::CHAR_ERROR,
        }
    }
}
