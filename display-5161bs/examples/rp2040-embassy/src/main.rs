// This example will display the numbers from 0 to 9 in a loop.
// Every other loop will have the DP on.
// So:
// 0
// 1
// (...)
// 9
// 0.
// 1.
// (...)
// 9.
// 0
// 1
// (...)

#![no_std]
#![no_main]

use display_5161bs::Display5161BS;
use embassy_executor::Spawner;
use embassy_rp::gpio::{self};
use embassy_time::{block_for, Duration};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_rp::init(Default::default());

    let mut e = Output::new(p.PIN_2, Level::Low);
    let mut d = Output::new(p.PIN_3, Level::Low);
    let mut vcc_1 = Output::new(p.PIN_4, Level::Low);
    let mut c = Output::new(p.PIN_5, Level::Low);
    let mut dp = Output::new(p.PIN_6, Level::Low);

    let mut b = Output::new(p.PIN_11, Level::Low);
    let mut a = Output::new(p.PIN_10, Level::Low);
    let mut vcc_2 = Output::new(p.PIN_9, Level::Low);
    let mut f = Output::new(p.PIN_8, Level::Low);
    let mut g = Output::new(p.PIN_7, Level::Low);

    let mut display = Display5161BS::new(
        &mut a,
        &mut b,
        &mut c,
        &mut d,
        &mut e,
        &mut f,
        &mut g,
        &mut dp,
        (&mut vcc_1, &mut vcc_2),
    );

    let mut i = 0_u8;
    let mut dp = false;
    loop {
        display.show_number_dot(i, dp);

        block_for(Duration::from_millis(300));

        i = (i + 1) % 10;
        if i == 0 {
            dp = !dp;
        }
    }
}
