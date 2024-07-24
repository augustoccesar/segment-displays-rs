//! Driver for using the 7-Segment Display 5161BS
//!
//! Example Embassy + RP2040:
//! ```
//! #![no_std]
//! #![no_main]

//! use embassy_executor::Spawner;
//! use embassy_rp::gpio::{self};
//! use embassy_time::{block_for, Duration};
//! use gpio::{Level, Output};
//! use {defmt_rtt as _, panic_probe as _};
//!
//! #[embassy_executor::main]
//! async fn main(_spawner: Spawner) -> ! {
//!     let p = embassy_rp::init(Default::default());
//!
//!     let mut e = Output::new(p.PIN_2, Level::Low);
//!     let mut d = Output::new(p.PIN_3, Level::Low);
//!     let mut vcc_1 = Output::new(p.PIN_4, Level::Low);
//!     let mut c = Output::new(p.PIN_5, Level::Low);
//!     let mut dp = Output::new(p.PIN_6, Level::Low);
//!
//!     let mut b = Output::new(p.PIN_11, Level::Low);
//!     let mut a = Output::new(p.PIN_10, Level::Low);
//!     let mut vcc_2 = Output::new(p.PIN_9, Level::Low);
//!     let mut f = Output::new(p.PIN_8, Level::Low);
//!     let mut g = Output::new(p.PIN_7, Level::Low);
//!
//!     let mut display = Display5161BS::new(
//!         &mut a,
//!         &mut b,
//!         &mut c,
//!         &mut d,
//!         &mut e,
//!         &mut f,
//!         &mut g,
//!         &mut dp,
//!         (&mut vcc_1, &mut vcc_2),
//!     );

//!     let mut i = 0_u8;
//!     let mut dp = false;
//!     loop {
//!         display.show_number_dot(i, dp);
//!
//!         block_for(Duration::from_millis(300));
//!
//!         i = (i + 1) % 10;
//!         if i == 0 {
//!             dp = !dp;
//!         }
//!     }
//! }
//! ```

#![no_std]

use display_shared::{Digit, Segment};
use embedded_hal::digital::OutputPin;

pub struct Display5161BS<'a, E: embedded_hal::digital::Error> {
    gpio_a: &'a mut dyn OutputPin<Error = E>,
    gpio_b: &'a mut dyn OutputPin<Error = E>,
    gpio_c: &'a mut dyn OutputPin<Error = E>,
    gpio_d: &'a mut dyn OutputPin<Error = E>,
    gpio_e: &'a mut dyn OutputPin<Error = E>,
    gpio_f: &'a mut dyn OutputPin<Error = E>,
    gpio_g: &'a mut dyn OutputPin<Error = E>,
    gpio_dp: &'a mut dyn OutputPin<Error = E>,
    vcc: (
        &'a mut dyn OutputPin<Error = E>,
        &'a mut dyn OutputPin<Error = E>,
    ),
}

impl<'a, E: embedded_hal::digital::Error> Display5161BS<'a, E> {
    pub fn new(
        gpio_a: &'a mut dyn OutputPin<Error = E>,
        gpio_b: &'a mut dyn OutputPin<Error = E>,
        gpio_c: &'a mut dyn OutputPin<Error = E>,
        gpio_d: &'a mut dyn OutputPin<Error = E>,
        gpio_e: &'a mut dyn OutputPin<Error = E>,
        gpio_f: &'a mut dyn OutputPin<Error = E>,
        gpio_g: &'a mut dyn OutputPin<Error = E>,
        gpio_dp: &'a mut dyn OutputPin<Error = E>,
        vcc: (
            &'a mut dyn OutputPin<Error = E>,
            &'a mut dyn OutputPin<Error = E>,
        ),
    ) -> Self {
        vcc.0.set_high().unwrap();
        vcc.1.set_high().unwrap();

        Self {
            gpio_a,
            gpio_b,
            gpio_c,
            gpio_d,
            gpio_e,
            gpio_f,
            gpio_g,
            gpio_dp,
            vcc,
        }
    }

    pub fn toggle_on(&mut self) {
        self.vcc.0.set_high().unwrap();
        self.vcc.1.set_high().unwrap();
    }

    pub fn toggle_off(&mut self) {
        self.vcc.0.set_low().unwrap();
        self.vcc.1.set_low().unwrap();
    }

    pub fn clear(&mut self) {
        self.gpio_a.set_high().unwrap();
        self.gpio_b.set_high().unwrap();
        self.gpio_c.set_high().unwrap();
        self.gpio_d.set_high().unwrap();
        self.gpio_e.set_high().unwrap();
        self.gpio_f.set_high().unwrap();
        self.gpio_g.set_high().unwrap();
        self.gpio_dp.set_high().unwrap();
    }

    pub fn show_digit(&mut self, digit: Digit) {
        self.clear();

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

    pub fn show_number(&mut self, number: u8) {
        let digit = Digit::from(number);

        self.show_digit(digit);
    }

    pub fn show_number_dot(&mut self, number: u8, with_dot: bool) {
        let mut digit = Digit::from(number);

        if with_dot {
            digit.toggle_dp();
        }

        self.show_digit(digit);
    }
}
