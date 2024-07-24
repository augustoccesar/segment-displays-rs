//! Driver for using the 4-Digit 7-Segment Display 3461BS
//!
//! Example Embassy + RP2040:
//! ```
//! #![no_std]
//! #![no_main]
//!
//! use display_3461bs::{Digit, Display3461bs, Position};
//! use embassy_executor::Spawner;
//! use embassy_rp::gpio::{self};
//! use embassy_time::{block_for, Duration, Instant};
//! use gpio::{Level, Output};
//! use {defmt_rtt as _, panic_probe as _};
//!
//! const UPDATE_FREQUENCY: Duration = Duration::from_millis(25);
//!
//! #[embassy_executor::main]
//! async fn main(_spawner: Spawner) -> ! {
//!     let p = embassy_rp::init(Default::default());
//!
//!     let mut e = Output::new(p.PIN_2, Level::Low);
//!     let mut d = Output::new(p.PIN_3, Level::Low);
//!     let mut dp = Output::new(p.PIN_4, Level::Low);
//!     let mut c = Output::new(p.PIN_5, Level::Low);
//!     let mut g = Output::new(p.PIN_6, Level::Low);
//!     let mut d4 = Output::new(p.PIN_7, Level::Low);
//!
//!     let mut b = Output::new(p.PIN_8, Level::Low);
//!     let mut d3 = Output::new(p.PIN_9, Level::Low);
//!     let mut d2 = Output::new(p.PIN_10, Level::Low);
//!     let mut f = Output::new(p.PIN_11, Level::Low);
//!     let mut a = Output::new(p.PIN_12, Level::Low);
//!     let mut d1 = Output::new(p.PIN_13, Level::Low);
//!
//!     let mut display = Display3461bs::new(
//!         &mut d1, &mut d2, &mut d3, &mut d4, &mut a, &mut b, &mut c, &mut d, &mut e, &mut f, &mut g,
//!         &mut dp,
//!     );
//!
//!     let mut last_update = Instant::now();
//!
//!     let mut number = 0_u16;
//!     loop {
//!         let first_number = number / 100;
//!         let second_number = (number % 100) / 10;
//!         let third_number = (number % 100) % 10;
//!
//!         display.show_digit(
//!             Position::First,
//!             Digit::new(true, true, true, false, true, true, true, true),
//!         );
//!         block_for(Duration::from_millis(5));
//!         display.show_number(Position::Second, first_number as u8);
//!         block_for(Duration::from_millis(5));
//!         display.show_number(Position::Third, second_number as u8);
//!         block_for(Duration::from_millis(5));
//!         display.show_number(Position::Fourth, third_number as u8);
//!         block_for(Duration::from_millis(5));
//!
//!         if (Instant::now() - last_update) > UPDATE_FREQUENCY {
//!             number = (number + 1) % 1000;
//!             last_update = Instant::now();
//!         }
//!     }
//! }
//! ```

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
