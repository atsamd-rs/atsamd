#![no_std]
#![no_main]

use atsame54_xpro as bsp;
use bsp::hal;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let pins = bsp::Pins::new(peripherals.PORT);
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
