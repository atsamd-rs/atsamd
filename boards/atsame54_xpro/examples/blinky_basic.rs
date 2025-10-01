#![no_std]
#![no_main]

use atsame54_xpro as bsp;
use bsp::hal;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use hal::clock::v2::clock_system_at_reset;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();

    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let (mut _buses, clocks, _tokens) = clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );

    let (mut delay, _glk0) = Delay::new(core.SYST, clocks.gclk0);

    let pins = bsp::Pins::new(peripherals.port);
    let mut led = bsp::pin_alias!(pins.led).into_push_pull_output();

    loop {
        delay.delay_ms(200u8);
        led.set_high().unwrap();
        rprintln!("LED OFF");
        delay.delay_ms(200u8);
        led.set_low().unwrap();
        rprintln!("LED ON");
    }
}
