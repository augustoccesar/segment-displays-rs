//! Shared code between display-5161bs and display3461bs
#![no_std]

use embedded_hal::digital::PinState;

/// Represents each segment on the 7-Segment digit.
///
/// The variants represents the segments on the following manner:
/// ```text
///       A                    
///     -----
///    |     |
///  F |     | B
///    |  G  |
///     -----
///    |     |
///  E |     | C
///    |  D  |
///     -----    □ DP
/// ```
#[derive(Clone, Debug)]
pub enum Segment {
    /// Top
    A,
    /// Top right
    B,
    /// Bottom right
    C,
    /// Bottom
    D,
    /// Bottom left
    E,
    /// Top left
    F,
    /// Middle
    G,
    /// Dot
    DP,
}

/// A group of 8 booleans representing the segments.
#[derive(Default, Clone, Debug)]
pub struct Digit {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
    dp: bool,
}

impl Digit {
    pub fn new(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool, dp: bool) -> Self {
        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            dp,
        }
    }

    /// Gets the [PinState] for a specific segment.
    ///
    /// On the displays the segments are turned on or off by the inverse of the boolean.
    /// So for ease of use, they are represented "naturally" on the [Digit], but when parsing it
    /// into a [PinState], we have to invert it.
    ///
    /// So, a segment as `true` is `ON` but is [PinState::Low].
    pub fn segment_pin_state(&self, segment: Segment) -> PinState {
        match segment {
            Segment::A => PinState::from(!self.a),
            Segment::B => PinState::from(!self.b),
            Segment::C => PinState::from(!self.c),
            Segment::D => PinState::from(!self.d),
            Segment::E => PinState::from(!self.e),
            Segment::F => PinState::from(!self.f),
            Segment::G => PinState::from(!self.g),
            Segment::DP => PinState::from(!self.dp),
        }
    }

    /// Toggles the dp (dot) on or off.
    pub fn toggle_dp(&mut self) {
        self.dp = !self.dp;
    }
}

impl From<u8> for Digit {
    fn from(value: u8) -> Self {
        let mut digit = Self::default();

        match value {
            0 => {
                digit.a = true;
                digit.b = true;
                digit.c = true;
                digit.d = true;
                digit.e = true;
                digit.f = true;
            }
            1 => {
                digit.b = true;
                digit.c = true;
            }
            2 => {
                digit.a = true;
                digit.b = true;
                digit.d = true;
                digit.e = true;
                digit.g = true;
            }
            3 => {
                digit.a = true;
                digit.b = true;
                digit.c = true;
                digit.d = true;
                digit.g = true;
            }
            4 => {
                digit.b = true;
                digit.c = true;
                digit.f = true;
                digit.g = true;
            }
            5 => {
                digit.a = true;
                digit.c = true;
                digit.d = true;
                digit.f = true;
                digit.g = true;
            }
            6 => {
                digit.a = true;
                digit.c = true;
                digit.d = true;
                digit.e = true;
                digit.f = true;
                digit.g = true;
            }
            7 => {
                digit.a = true;
                digit.b = true;
                digit.c = true;
            }
            8 => {
                digit.a = true;
                digit.b = true;
                digit.c = true;
                digit.d = true;
                digit.e = true;
                digit.f = true;
                digit.g = true;
            }
            9 => {
                digit.a = true;
                digit.b = true;
                digit.c = true;
                digit.d = true;
                digit.f = true;
                digit.g = true;
            }
            _ => panic!("Invalid u16. Must be 0..=9"), // TODO: Maybe implement TryFrom instead?
        }

        return digit;
    }
}
