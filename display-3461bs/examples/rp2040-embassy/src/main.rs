// This example will loop and show on the display the values from A.000 to A.999.
// So:
//  A.000
//  A.001
//  A.002
//  (...)
//  A.009
//  A.010
//  A.011
//  (...)
//  A.099
//  A.100
//  A.101
//  (...)
//  A.999
//  A.000
//  (...)

#![no_std]
#![no_main]

use display_3461bs::{Digit, Display3461bs, Position};
use embassy_executor::Spawner;
use embassy_rp::gpio::{self};
use embassy_time::{block_for, Duration, Instant};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

const UPDATE_FREQUENCY: Duration = Duration::from_millis(25);

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_rp::init(Default::default());

    let mut e = Output::new(p.PIN_2, Level::Low);
    let mut d = Output::new(p.PIN_3, Level::Low);
    let mut dp = Output::new(p.PIN_4, Level::Low);
    let mut c = Output::new(p.PIN_5, Level::Low);
    let mut g = Output::new(p.PIN_6, Level::Low);
    let mut d4 = Output::new(p.PIN_7, Level::Low);

    let mut b = Output::new(p.PIN_8, Level::Low);
    let mut d3 = Output::new(p.PIN_9, Level::Low);
    let mut d2 = Output::new(p.PIN_10, Level::Low);
    let mut f = Output::new(p.PIN_11, Level::Low);
    let mut a = Output::new(p.PIN_12, Level::Low);
    let mut d1 = Output::new(p.PIN_13, Level::Low);

    let mut display = Display3461bs::new(
        &mut d1, &mut d2, &mut d3, &mut d4, &mut a, &mut b, &mut c, &mut d, &mut e, &mut f, &mut g,
        &mut dp,
    );

    let mut last_update = Instant::now();

    let mut number = 0_u16;
    loop {
        let first_number = number / 100;
        let second_number = (number % 100) / 10;
        let third_number = (number % 100) % 10;

        display.show_digit(
            Position::First,
            Digit::new(true, true, true, false, true, true, true, true),
        );
        block_for(Duration::from_millis(5));
        display.show_number(Position::Second, first_number as u8);
        block_for(Duration::from_millis(5));
        display.show_number(Position::Third, second_number as u8);
        block_for(Duration::from_millis(5));
        display.show_number(Position::Fourth, third_number as u8);
        block_for(Duration::from_millis(5));

        if (Instant::now() - last_update) > UPDATE_FREQUENCY {
            number = (number + 1) % 1000;
            last_update = Instant::now();
        }
    }
}
