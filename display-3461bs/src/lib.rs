#![no_std]

use display_shared::Segment;
use embedded_hal::digital::OutputPin;

pub use display_shared::Digit;

pub struct Display3461bs<'a, E: embedded_hal::digital::Error> {
    vcc_d1: &'a mut dyn OutputPin<Error = E>,
    vcc_d2: &'a mut dyn OutputPin<Error = E>,
    vcc_d3: &'a mut dyn OutputPin<Error = E>,
    vcc_d4: &'a mut dyn OutputPin<Error = E>,
    gpio_a: &'a mut dyn OutputPin<Error = E>,
    gpio_b: &'a mut dyn OutputPin<Error = E>,
    gpio_c: &'a mut dyn OutputPin<Error = E>,
    gpio_d: &'a mut dyn OutputPin<Error = E>,
    gpio_e: &'a mut dyn OutputPin<Error = E>,
    gpio_f: &'a mut dyn OutputPin<Error = E>,
    gpio_g: &'a mut dyn OutputPin<Error = E>,
    gpio_dp: &'a mut dyn OutputPin<Error = E>,
}

pub enum Position {
    First,
    Second,
    Third,
    Fourth,
}

impl<'a, E: embedded_hal::digital::Error> Display3461bs<'a, E> {
    pub fn new(
        vcc_d1: &'a mut dyn OutputPin<Error = E>,
        vcc_d2: &'a mut dyn OutputPin<Error = E>,
        vcc_d3: &'a mut dyn OutputPin<Error = E>,
        vcc_d4: &'a mut dyn OutputPin<Error = E>,
        gpio_a: &'a mut dyn OutputPin<Error = E>,
        gpio_b: &'a mut dyn OutputPin<Error = E>,
        gpio_c: &'a mut dyn OutputPin<Error = E>,
        gpio_d: &'a mut dyn OutputPin<Error = E>,
        gpio_e: &'a mut dyn OutputPin<Error = E>,
        gpio_f: &'a mut dyn OutputPin<Error = E>,
        gpio_g: &'a mut dyn OutputPin<Error = E>,
        gpio_dp: &'a mut dyn OutputPin<Error = E>,
    ) -> Self {
        Self {
            vcc_d1,
            vcc_d2,
            vcc_d3,
            vcc_d4,
            gpio_a,
            gpio_b,
            gpio_c,
            gpio_d,
            gpio_e,
            gpio_f,
            gpio_g,
            gpio_dp,
        }
    }

    pub fn clear(&mut self) {
        self.vcc_d1.set_low().unwrap();
        self.vcc_d2.set_low().unwrap();
        self.vcc_d3.set_low().unwrap();
        self.vcc_d4.set_low().unwrap();

        self.gpio_a.set_high().unwrap();
        self.gpio_b.set_high().unwrap();
        self.gpio_c.set_high().unwrap();
        self.gpio_d.set_high().unwrap();
        self.gpio_e.set_high().unwrap();
        self.gpio_f.set_high().unwrap();
        self.gpio_g.set_high().unwrap();
        self.gpio_dp.set_high().unwrap();
    }

    pub fn show_digit(&mut self, position: Position, digit: Digit) {
        self.clear();

        match position {
            Position::First => &mut self.vcc_d1,
            Position::Second => &mut self.vcc_d2,
            Position::Third => &mut self.vcc_d3,
            Position::Fourth => &mut self.vcc_d4,
        }
        .set_high()
        .unwrap();

        self.gpio_a
            .set_state(digit.segment_pin_state(Segment::A))
            .unwrap();
        self.gpio_b
            .set_state(digit.segment_pin_state(Segment::B))
            .unwrap();
        self.gpio_c
            .set_state(digit.segment_pin_state(Segment::C))
            .unwrap();
        self.gpio_d
            .set_state(digit.segment_pin_state(Segment::D))
            .unwrap();
        self.gpio_e
            .set_state(digit.segment_pin_state(Segment::E))
            .unwrap();
        self.gpio_f
            .set_state(digit.segment_pin_state(Segment::F))
            .unwrap();
        self.gpio_g
            .set_state(digit.segment_pin_state(Segment::G))
            .unwrap();
        self.gpio_dp
            .set_state(digit.segment_pin_state(Segment::DP))
            .unwrap();
    }

    pub fn show_number(&mut self, position: Position, number: u8) {
        let digit = Digit::from(number);

        self.show_digit(position, digit);
    }
}
